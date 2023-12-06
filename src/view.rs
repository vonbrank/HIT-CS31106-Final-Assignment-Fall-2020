mod about;
mod contact_detail;
mod contact_list;
mod create_contact;
mod create_phone_book;
pub mod home_entry;
mod phone_book_list;
pub mod settings;

use std::{fmt::Debug, usize};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind},
    style::Stylize,
};

use crate::{
    model::{Contact, Model, SettingsState},
    utils::{align_key_value, align_string, AlignType},
};

use self::{
    about::About,
    contact_detail::ContactDetail,
    contact_list::ContactListPage,
    create_contact::CreateContactPage,
    create_phone_book::CreatePhoneBookPage,
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
    NewContact(String),
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
            PageType::NewPhoneBook => Box::new(CreatePhoneBookPage::new(
                model
                    .phone_books
                    .iter()
                    .map(|item| item.name.clone())
                    .collect(),
            )),
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
            PageType::NewContact(phone_book_name) => {
                let phone_book = model
                    .phone_books
                    .iter()
                    .find(|item| item.name.eq(phone_book_name))
                    .unwrap();
                Box::new(CreateContactPage::new(
                    phone_book_name.clone(),
                    phone_book
                        .contacts
                        .iter()
                        .map(|item| item.name.clone())
                        .collect(),
                ))
            }
            PageType::About => Box::new(About::new()),
            _ => Box::new(EmptyPage {}),
        }
    }
}

pub enum UiElement {
    Text(String, AlignType),
    KeyValue((String, String), AlignType),
    TextList(Vec<String>, usize, AlignType),
    KeyValueList(Vec<(String, String)>, usize, AlignType),
}

pub struct PageContent {
    ui_elements: Vec<UiElement>,
    inner_width: u32,
    inner_height: u32,
}

impl PageContent {
    pub fn new() -> PageContent {
        PageContent {
            ui_elements: vec![],
            inner_width: 34,
            inner_height: 10,
        }
    }
    pub fn from_list(
        name: String,
        list: Vec<String>,
        selected_index: usize,
        align_type: AlignType,
    ) -> PageContent {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text(name, AlignType::Center));
        page_content.add_element(UiElement::Text("-".repeat(128), align_type));
        page_content.add_element(UiElement::TextList(list, selected_index, align_type));
        page_content
    }

    pub fn from_lines(name: String, lines: Vec<String>, align_type: AlignType) -> PageContent {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text(name, AlignType::Center));
        page_content.add_element(UiElement::Text("-".repeat(128), align_type));
        for item in lines {
            page_content.add_element(UiElement::Text(item, align_type));
        }
        page_content
    }

    pub fn add_element(&mut self, element: UiElement) {
        self.ui_elements.push(element);
    }
    pub fn render(&self) {
        println!("+{}+", "-".repeat(self.inner_width as usize));

        let mut line_number = 0;

        for ui_element in self.ui_elements.iter() {
            match ui_element {
                UiElement::Text(text, align_type) => {
                    println!("|{}|", align_string(text, self.inner_width, *align_type));
                    line_number += 1;
                }
                UiElement::KeyValue((key, value), align_type) => {
                    println!(
                        "|{}|",
                        align_key_value(key, value, self.inner_width, *align_type)
                    );
                    line_number += 1;
                }
                UiElement::TextList(list, select_index, align_type) => {
                    for (index, item) in list.into_iter().enumerate() {
                        if index == *select_index {
                            println!(
                                "|{}|",
                                align_string(item, self.inner_width, *align_type)
                                    .black()
                                    .on_white()
                            );
                        } else {
                            println!("|{}|", align_string(item, self.inner_width, *align_type));
                        }
                        line_number += 1;
                    }
                }
                UiElement::KeyValueList(entries, select_index, align_type) => {
                    for (index, (key, value)) in entries.into_iter().enumerate() {
                        if index == *select_index {
                            println!(
                                "|{}|",
                                align_key_value(key, value, self.inner_width, *align_type)
                                    .black()
                                    .on_white()
                            );
                        } else {
                            println!(
                                "|{}|",
                                align_key_value(key, value, self.inner_width, *align_type)
                            );
                        }
                        line_number += 1;
                    }
                }
            }
        }

        for _ in 0..(self.inner_height - line_number as u32) {
            println!("|{}|", " ".repeat(self.inner_width as usize));
        }

        println!("+{}+", "-".repeat(self.inner_width as usize));
    }
}

pub enum Action {
    // HomeEntry(HomeEntryAction),
    Navigate(PageType),
    UpdateSettings(SettingsState, SettingsPageSaved),
    CreateNewPhoneBook(String),
    CreateNewContact(String, Contact),
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
    if target_list.len() <= 0 {
        return false;
    }
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
    if target_list.len() <= 0 {
        return false;
    }

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
