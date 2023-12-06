use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::{handle_vertical_scroll, Action, PageContent, PageTrait};

pub struct PhoneBookListPage {
    page_content: PageContent,
    current_select_index: usize,
    phone_book_name_list: Vec<String>,
}

impl PhoneBookListPage {
    pub fn new(phone_book_list: Vec<String>) -> PhoneBookListPage {
        let mut phone_book_list_page = PhoneBookListPage {
            page_content: PageContent::new(),
            current_select_index: 0,
            phone_book_name_list: phone_book_list,
        };

        phone_book_list_page.refresh_content();

        phone_book_list_page
    }

    fn refresh_content(&mut self) {
        let page_content = if self.phone_book_name_list.is_empty() {
            PageContent::from_lines(
                "Phone Book List".to_string(),
                vec![
                    "There is no phone books.".to_string(),
                    "Go back to create a new one.".to_string(),
                ],
                super::AlignType::Left,
            )
        } else {
            PageContent::from_list(
                "Phone Book List".to_string(),
                self.phone_book_name_list.clone(),
                self.current_select_index,
                super::AlignType::Left,
            )
        };

        self.page_content = page_content;
    }
}

impl PageTrait for PhoneBookListPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        if handle_vertical_scroll(
            key_event,
            &self.phone_book_name_list,
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
                        if let Some(current_phone_book_name) =
                            self.phone_book_name_list.get(self.current_select_index)
                        {
                            action = Action::Navigate(super::PageType::PhoneBook(
                                current_phone_book_name.clone(),
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
