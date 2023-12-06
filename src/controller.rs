use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    app::SavingData,
    model::{Model, ModelPersist, PhoneBook, SettingsState},
    view::{home_entry::HomeEntryAction, Action, PageTrait, PageType},
};

pub struct Controller {
    keyboard_event_receiver: UnboundedReceiver<KeyEvent>,
    current_page_type: PageType,
    page_cache: HashMap<PageType, Box<dyn PageTrait>>,
    model: Model,
    saving_data_sender: UnboundedSender<SavingData>,
}

impl Controller {
    pub fn new(
        keyboard_event_receiver: UnboundedReceiver<KeyEvent>,
        saving_data_sender: UnboundedSender<SavingData>,
    ) -> Controller {
        Controller {
            keyboard_event_receiver,
            saving_data_sender,
            current_page_type: PageType::HomeEntry,
            page_cache: HashMap::new(),
            model: Model::new(),
        }
    }

    pub fn from_saved_data(
        keyboard_event_receiver: UnboundedReceiver<KeyEvent>,
        saving_data_sender: UnboundedSender<SavingData>,
        phone_books: Vec<PhoneBook>,
        settings: SettingsState,
    ) -> Controller {
        Controller {
            keyboard_event_receiver,
            saving_data_sender,
            current_page_type: PageType::HomeEntry,
            page_cache: HashMap::new(),
            model: Model {
                phone_books,
                settings,
                persist: ModelPersist::new(),
            },
        }
    }

    pub async fn update(&mut self) -> UpdateResult {
        let mut update_result = UpdateResult::Continue;

        let action = self.handle_input().await;

        match action {
            Action::Navigate(page_type) => {
                self.current_page_type = page_type;
            }
            Action::UpdateSettings(settings_state, saved) => {
                self.model.settings = settings_state;
                let _ = self
                    .saving_data_sender
                    .send(SavingData::from_model(&self.model));
                self.page_cache.remove(&PageType::Settings);
                self.model.persist.settings_page = Some(saved);
            }
            Action::CreateNewPhoneBook(new_phone_book_name) => {
                self.model.phone_books.push(PhoneBook {
                    name: new_phone_book_name.clone(),
                    contacts: vec![],
                });
                let _ = self
                    .saving_data_sender
                    .send(SavingData::from_model(&self.model));
                self.page_cache.remove(&PageType::PhoneBookList);
                self.page_cache.remove(&PageType::NewPhoneBook);
                self.current_page_type = PageType::PhoneBook(new_phone_book_name);
            }
            Action::CreateNewContact(phone_book_name, contact) => {
                let phone_book = self
                    .model
                    .phone_books
                    .iter_mut()
                    .find(|item| item.name.eq(&phone_book_name))
                    .unwrap();
                phone_book.contacts.push(contact);
                let _ = self
                    .saving_data_sender
                    .send(SavingData::from_model(&self.model));
                self.page_cache
                    .remove(&PageType::PhoneBook(phone_book_name.clone()));
                self.page_cache
                    .remove(&PageType::NewContact(phone_book_name.clone()));
                self.current_page_type = PageType::PhoneBook(phone_book_name);
            }
            Action::Exit => match self.current_page_type {
                PageType::HomeEntry => update_result = UpdateResult::Exit,
                PageType::PhoneBookList
                | PageType::PhoneBook(_)
                | PageType::NewPhoneBook
                | PageType::Settings
                | PageType::About => {
                    if self.current_page_type == PageType::Settings {
                        self.page_cache.remove(&PageType::Settings);
                        self.model.persist.settings_page = None;
                    }

                    self.current_page_type = PageType::HomeEntry
                }
                _ => {}
            },
            _ => {}
        }

        self.render().await;

        update_result
    }

    async fn handle_input(&mut self) -> Action {
        if let Some(key_event) = self.keyboard_event_receiver.recv().await {
            let current_page_view = self.page_cache.get_mut(&self.current_page_type).unwrap();
            current_page_view.handle_input(key_event)
        } else {
            Action::None
        }
    }

    pub async fn render(&mut self) {
        print!("\x1B[2J\x1B[1;1H");

        let current_page_view = self.page_cache.get(&self.current_page_type);
        if let Some(page_view) = current_page_view {
            page_view.render();
        } else {
            let page_view = self
                .current_page_type
                .create_page_from_model(&self.model)
                .await;
            page_view.render();
            self.page_cache
                .insert(self.current_page_type.clone(), page_view);
        }
    }

    pub fn close(&mut self) {
        self.keyboard_event_receiver.close();
        println!("Press any key to exit.");
    }
}

pub enum UpdateResult {
    Continue,
    Exit,
}
