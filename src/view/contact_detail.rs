use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Contact;

use super::{Action, PageContent, PageTrait};

pub struct ContactDetail {
    page_content: PageContent,
    phone_book_name: String,
    data: Contact,
}

impl ContactDetail {
    pub fn new(phone_book_name: String, data: Contact) -> ContactDetail {
        let mut contact_detail = ContactDetail {
            page_content: PageContent::new(),
            data,
            phone_book_name,
        };

        contact_detail.refresh_content();

        contact_detail
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_lines(
            format!("Contact - {}", self.data.name),
            vec![
                format!("Name: {}", self.data.name),
                format!("Phone Number: {}", self.data.phone_number),
            ],
        );
    }
}

impl PageTrait for ContactDetail {
    fn handle_input(&mut self, key_event: crossterm::event::KeyEvent) -> super::Action {
        let mut action = Action::None;
        match key_event {
            KeyEvent {
                kind: KeyEventKind::Press,
                ..
            } => match key_event {
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
        action
    }

    fn render(&self) {
        self.page_content.render();
    }
}
