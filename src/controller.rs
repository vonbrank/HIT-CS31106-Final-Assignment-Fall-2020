use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    model::{Model, PhoneBook},
    view::{home_entry::HomeEntryAction, Action, PageTrait, PageType},
};

pub struct Controller {
    receiver: UnboundedReceiver<KeyEvent>,
    current_page_type: PageType,
    page_cache: HashMap<PageType, Box<dyn PageTrait>>,
    model: Model,
}

impl Controller {
    pub fn new(receiver: UnboundedReceiver<KeyEvent>) -> Controller {
        Controller {
            receiver,
            current_page_type: PageType::HomeEntry,
            page_cache: HashMap::new(),
            model: Model::from_fake_data(),
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
                self.page_cache.remove(&PageType::Settings);
                self.model.persist.settings_page = Some(saved);
            }
            Action::CreateNewPhoneBook(new_phone_book_name) => {
                self.model.phone_books.push(PhoneBook {
                    name: new_phone_book_name.clone(),
                    contacts: vec![],
                });
                self.page_cache.remove(&PageType::PhoneBookList);
                self.page_cache.remove(&PageType::NewPhoneBook);
                self.current_page_type = PageType::PhoneBook(new_phone_book_name)
            }
            Action::Exit => match self.current_page_type {
                PageType::HomeEntry => update_result = UpdateResult::Exit,
                PageType::PhoneBookList
                | PageType::PhoneBook(_)
                | PageType::NewPhoneBook
                | PageType::Settings => {
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
        if let Some(key_event) = self.receiver.recv().await {
            let current_page_view = self.page_cache.get_mut(&self.current_page_type).unwrap();
            current_page_view.handle_input(key_event)
        } else {
            Action::None
        }
    }

    pub async fn render(&mut self) {
        print!("\x1B[2J\x1B[1;1H");

        // println!("fuck");

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
        self.receiver.close();
    }
}

pub enum UpdateResult {
    Continue,
    Exit,
}
