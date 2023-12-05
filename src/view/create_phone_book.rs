use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::{Action, PageContent, PageTrait};

pub struct CreatePhoneBookPage {
    page_content: PageContent,
    input_name: String,
}

impl CreatePhoneBookPage {
    pub fn new() -> CreatePhoneBookPage {
        let mut create_phone_book_page = CreatePhoneBookPage {
            page_content: PageContent::new(),
            input_name: "".to_string(),
        };

        create_phone_book_page.refresh_content();

        create_phone_book_page
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_list(
            "Create Phone Book".to_string(),
            vec!["Name: ".to_string(), self.input_name.clone()],
            1,
        );
    }
}

impl PageTrait for CreatePhoneBookPage {
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
                } => {
                    self.input_name = format!("{}{}", self.input_name.clone(), ch);
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    if self.input_name.len() >= 1 {
                        self.input_name.remove(self.input_name.len() - 1);
                    }
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    action = Action::CreateNewPhoneBook(self.input_name.clone());
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => action = Action::Exit,
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
