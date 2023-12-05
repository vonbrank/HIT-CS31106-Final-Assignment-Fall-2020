use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Contact;

use super::{Action, PageContent, PageTrait};

enum FocusType {
    Name,
    PhoneNumber,
}

#[derive(Clone)]
enum InputInvalid {
    None,
    Repeat,
    PhoneNumberIsNotNumber,
    Empty(String),
}

pub struct CreateContactPage {
    page_content: PageContent,
    phone_book_name: String,
    input_name: String,
    input_phone_number: String,
    forcus_type: FocusType,
    existed_list: Vec<String>,
    input_invalid: InputInvalid,
}

impl CreateContactPage {
    pub fn new(phone_book_name: String, existed_list: Vec<String>) -> CreateContactPage {
        let mut create_phone_book_page = CreateContactPage {
            page_content: PageContent::new(),
            input_name: "".to_string(),
            input_phone_number: "".to_string(),
            phone_book_name,
            forcus_type: FocusType::Name,
            input_invalid: InputInvalid::None,
            existed_list,
        };

        create_phone_book_page.refresh_content();

        create_phone_book_page
    }

    fn refresh_content(&mut self) {
        let mut page_content = PageContent::from_list(
            "Create Phone Book".to_string(),
            vec![
                "Name: ".to_string(),
                self.input_name.clone(),
                "Phone Number: ".to_string(),
                self.input_phone_number.clone(),
            ],
            match self.forcus_type {
                FocusType::Name => 1,
                FocusType::PhoneNumber => 3,
            },
            super::AlignType::Left,
        );

        match &self.input_invalid {
            InputInvalid::Repeat => {
                page_content.add_element(super::UiElement::Text(
                    format!("{} has been existed.", self.input_name),
                    crate::utils::AlignType::Left,
                ));
            }
            InputInvalid::PhoneNumberIsNotNumber => {
                page_content.add_element(super::UiElement::Text(
                    format!("{} is not a phone number.", self.input_phone_number),
                    crate::utils::AlignType::Left,
                ));
            }
            InputInvalid::Empty(field_name) => {
                page_content.add_element(super::UiElement::Text(
                    format!("{} cannot be empty.", field_name.clone()),
                    crate::utils::AlignType::Left,
                ));
            }
            InputInvalid::None => {}
        }

        self.page_content = page_content;
    }
}

impl PageTrait for CreateContactPage {
    fn handle_input(&mut self, key_event: crossterm::event::KeyEvent) -> super::Action {
        let mut action = Action::None;

        // let mut input_invalid = InputInvalid::None;

        match key_event {
            KeyEvent {
                kind: KeyEventKind::Press,
                ..
            } => match key_event {
                KeyEvent {
                    code: KeyCode::Char(ch),
                    ..
                } => {
                    match self.forcus_type {
                        FocusType::Name => {
                            self.input_name = format!("{}{}", self.input_name.clone(), ch);
                        }
                        FocusType::PhoneNumber => {
                            self.input_phone_number =
                                format!("{}{}", self.input_phone_number.clone(), ch);
                        }
                    }
                    self.input_invalid = InputInvalid::None;
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    match self.forcus_type {
                        FocusType::Name => {
                            if self.input_name.len() >= 1 {
                                self.input_name.remove(self.input_name.len() - 1);
                            }
                        }
                        FocusType::PhoneNumber => {
                            if self.input_phone_number.len() >= 1 {
                                self.input_phone_number
                                    .remove(self.input_phone_number.len() - 1);
                            }
                        }
                    }
                    self.input_invalid = InputInvalid::None;
                }
                KeyEvent {
                    code: KeyCode::Tab, ..
                } => match self.forcus_type {
                    FocusType::Name => self.forcus_type = FocusType::PhoneNumber,
                    FocusType::PhoneNumber => self.forcus_type = FocusType::Name,
                },
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    if self.existed_list.contains(&self.input_name) {
                        self.input_invalid = InputInvalid::Repeat;
                    } else if !self.input_phone_number.chars().all(|c| c.is_digit(10)) {
                        self.input_invalid = InputInvalid::PhoneNumberIsNotNumber;
                    } else if self.input_name.is_empty() {
                        self.input_invalid = InputInvalid::Empty("Name".to_string());
                    } else if self.input_phone_number.is_empty() {
                        self.input_invalid = InputInvalid::Empty("Phone Number".to_string());
                    } else {
                        action = Action::CreateNewContact(
                            self.phone_book_name.clone(),
                            Contact {
                                name: self.input_name.clone(),
                                phone_number: self.input_phone_number.clone(),
                            },
                        );
                    }
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    action =
                        Action::Navigate(super::PageType::PhoneBook(self.phone_book_name.clone()))
                }
                _ => {}
            },
            _ => {}
        }
        // self.input_invalid = input_invalid;
        self.refresh_content();

        action
    }

    fn render(&self) {
        self.page_content.render();
    }
}
