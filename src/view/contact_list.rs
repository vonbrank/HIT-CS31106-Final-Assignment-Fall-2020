use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Contact;

use super::{handle_vertical_scroll, Action, PageContent, PageTrait};

pub struct ContactListPage {
    page_content: PageContent,
    current_select_index: usize,
    contact_list: Vec<Contact>,
    phone_book_name: String,
}

impl ContactListPage {
    pub fn new(phone_book_name: String, contact_list: Vec<Contact>) -> ContactListPage {
        let mut phone_book_list_page = ContactListPage {
            page_content: PageContent::new(),
            current_select_index: 0,
            contact_list,
            phone_book_name,
        };

        phone_book_list_page.refresh_content();

        phone_book_list_page
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_list(
            format!("Phone Book - {}", self.phone_book_name),
            self.get_string_list_with_add_new(),
            self.current_select_index,
        );
    }

    fn get_string_list_with_add_new(&self) -> Vec<String> {
        let mut contact_list_string: Vec<String> = self
            .contact_list
            .iter()
            .map(|item| item.name.clone())
            .collect();
        contact_list_string.push("New Contact".to_string());
        contact_list_string
    }
}

impl PageTrait for ContactListPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        if handle_vertical_scroll(
            key_event,
            &self.get_string_list_with_add_new(),
            &mut self.current_select_index,
        ) {
            self.refresh_content();
        } else {
            match key_event {
                KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                } => match key_event {
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        if self.current_select_index
                            == self.get_string_list_with_add_new().len() - 1
                        {
                            action = Action::Navigate(super::PageType::NewContact(
                                self.phone_book_name.clone(),
                            ))
                        } else {
                            action = Action::Navigate(super::PageType::Contact(
                                self.phone_book_name.clone(),
                                self.contact_list
                                    .get(self.current_select_index)
                                    .unwrap()
                                    .name
                                    .clone(),
                            ))
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => {
                        action = Action::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        action
    }
    fn render(&self) {
        self.page_content.render();
    }
}
