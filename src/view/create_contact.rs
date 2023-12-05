use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Contact;

use super::{Action, PageContent, PageTrait};

enum FocusType {
    Name,
    PhoneNumber,
}

pub struct CreateContactPage {
    page_content: PageContent,
    phone_book_name: String,
    input_name: String,
    input_phone_number: String,
    forcus_type: FocusType,
}

impl CreateContactPage {
    pub fn new(phone_book_name: String) -> CreateContactPage {
        let mut create_phone_book_page = CreateContactPage {
            page_content: PageContent::new(),
            input_name: "".to_string(),
            input_phone_number: "".to_string(),
            phone_book_name,
            forcus_type: FocusType::Name,
        };

        create_phone_book_page.refresh_content();

        create_phone_book_page
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_list(
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
        );
    }
}

impl PageTrait for CreateContactPage {
    fn handle_input(&mut self, key_event: crossterm::event::KeyEvent) -> super::Action {
        let mut action = Action::None;

        match key_event {
            KeyEvent {
                kind: KeyEventKind::Press,
                ..
            } => match key_event {
                KeyEvent {
                    code: KeyCode::Char(ch),
                    ..
                } => match self.forcus_type {
                    FocusType::Name => {
                        self.input_name = format!("{}{}", self.input_name.clone(), ch);
                    }
                    FocusType::PhoneNumber => {
                        self.input_phone_number =
                            format!("{}{}", self.input_phone_number.clone(), ch);
                    }
                },
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => match self.forcus_type {
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
                },
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
                    action = Action::CreateNewContact(
                        self.phone_book_name.clone(),
                        Contact {
                            name: self.input_name.clone(),
                            phone_number: self.input_phone_number.clone(),
                        },
                    );
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

        self.refresh_content();

        action
    }

    fn render(&self) {
        self.page_content.render();
    }
}
