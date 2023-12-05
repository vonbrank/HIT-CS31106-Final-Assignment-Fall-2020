mod contact_detail;
mod contact_list;
pub mod home_entry;
mod phone_book_list;
pub mod settings;

use std::{fmt::Debug, usize};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::{Model, SettingsState};

use self::{
    contact_detail::ContactDetail,
    contact_list::ContactListPage,
    home_entry::{HomeEntry, HomeEntryAction},
    phone_book_list::PhoneBookListPage,
    settings::{SettingsPage, SettingsPageSaved},
};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PageType {
    HomeEntry,
    Settings,
    About,
    NewPhoneBook,
    PhoneBookList,
    PhoneBook(String),
    Contact(String, String),
}

impl PageType {
    pub async fn create_page(&self) -> Box<dyn PageTrait> {
        match self {
            PageType::HomeEntry => Box::new(HomeEntry::new().await),
            _ => Box::new(EmptyPage {}),
        }
    }

    pub async fn create_page_from_model(&self, model: &Model) -> Box<dyn PageTrait> {
        match self {
            PageType::HomeEntry => self.create_page().await,
            PageType::PhoneBookList => {
                let phone_book_list: Vec<String> = model
                    .phone_books
                    .iter()
                    .map(|item| item.name.clone())
                    .collect();
                let phone_book_list_page = PhoneBookListPage::new(phone_book_list);
                Box::new(phone_book_list_page)
            }
            PageType::PhoneBook(phone_book_name) => {
                let phone_book = model
                    .phone_books
                    .iter()
                    .find(|item| item.name.eq(phone_book_name))
                    .unwrap();
                let contact_list_page =
                    ContactListPage::new(phone_book_name.clone(), phone_book.contacts.clone());
                Box::new(contact_list_page)
            }
            PageType::Settings => {
                if let Some(saved) = model.persist.settings_page.clone() {
                    Box::new(SettingsPage::restore(model.settings.clone(), saved))
                } else {
                    Box::new(SettingsPage::new(model.settings.clone()))
                }
            }
            PageType::Contact(phone_book_name, contact_name) => {
                let phone_book = model
                    .phone_books
                    .iter()
                    .find(|item| item.name.eq(phone_book_name))
                    .unwrap();
                let contact = phone_book
                    .contacts
                    .iter()
                    .find(|item| item.name.eq(contact_name))
                    .unwrap();
                Box::new(ContactDetail::new(
                    phone_book_name.to_string(),
                    contact.clone(),
                ))
            }
            _ => Box::new(EmptyPage {}),
        }
    }
}

pub enum UiElement {
    Text(String),
    KeyValue((String, String)),
    TextList(Vec<String>, usize),
    KeyValueList(Vec<(String, String)>, usize),
}

pub struct PageContent {
    ui_elements: Vec<UiElement>,
}

impl PageContent {
    pub fn new() -> PageContent {
        PageContent {
            ui_elements: vec![],
        }
    }
    pub fn from_list(name: String, list: Vec<String>, selected_index: usize) -> PageContent {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text(name));
        page_content.add_element(UiElement::Text("----------".to_string()));
        page_content.add_element(UiElement::TextList(list, selected_index));
        page_content
    }

    pub fn from_lines(name: String, lines: Vec<String>) -> PageContent {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text(name));
        page_content.add_element(UiElement::Text("----------".to_string()));
        for item in lines {
            page_content.add_element(UiElement::Text(item));
        }
        page_content
    }

    pub fn add_element(&mut self, element: UiElement) {
        self.ui_elements.push(element);
    }
    pub fn render(&self) {
        for ui_element in self.ui_elements.iter() {
            match ui_element {
                UiElement::Text(text) => {
                    println!("{}", text);
                }
                UiElement::KeyValue((key, value)) => {
                    println!("{} {}", key, value);
                }
                UiElement::TextList(list, select_index) => {
                    for (index, item) in list.into_iter().enumerate() {
                        if index == *select_index {
                            println!("\x1B[7m{}\x1B[0m", item);
                        } else {
                            println!("{}", item);
                        }
                    }
                }
                UiElement::KeyValueList(entries, select_index) => {
                    for (index, (key, value)) in entries.into_iter().enumerate() {
                        if index == *select_index {
                            println!("\x1B[7m{} {}\x1B[0m", key, value);
                        } else {
                            println!("{} {}", key, value);
                        }
                    }
                }
            }
        }
    }
}

pub enum Action {
    // HomeEntry(HomeEntryAction),
    Navigate(PageType),
    UpdateSettings(SettingsState, SettingsPageSaved),
    Exit,
    None,
}

pub trait PageTrait {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action;
    fn render(&self);
}

pub struct EmptyPage {}

impl PageTrait for EmptyPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        Action::None
    }

    fn render(&self) {}
}

fn handle_vertical_scroll<T>(
    key_event: KeyEvent,
    target_list: &Vec<T>,
    seleced_index: &mut usize,
) -> bool {
    let mut res = false;

    match key_event {
        KeyEvent {
            kind: KeyEventKind::Press,
            ..
        } => match key_event {
            KeyEvent {
                code: KeyCode::Up, ..
            } => {
                if *seleced_index <= 0 {
                    *seleced_index = target_list.len() - 1;
                } else {
                    *seleced_index -= 1;
                }
                res = true;
            }
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => {
                if *seleced_index >= target_list.len() - 1 {
                    *seleced_index = 0;
                } else {
                    *seleced_index += 1;
                }
                res = true;
            }
            _ => {}
        },
        _ => {}
    }

    res
}

fn handle_horizontal_scroll<T: Debug>(
    key_event: KeyEvent,
    target_list: &Vec<T>,
    seleced_index: &mut usize,
) -> bool {
    let mut res = false;

    // println!("target: {:?}, index_before {}", target_list, seleced_index);

    match key_event {
        KeyEvent {
            kind: KeyEventKind::Press,
            ..
        } => match key_event {
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => {
                if *seleced_index <= 0 {
                    *seleced_index = target_list.len() - 1;
                } else {
                    *seleced_index -= 1;
                }
                res = true;
            }
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => {
                if *seleced_index >= target_list.len() - 1 {
                    *seleced_index = 0;
                } else {
                    *seleced_index += 1;
                }
                res = true;
            }
            _ => {}
        },
        _ => {}
    }

    res
}
