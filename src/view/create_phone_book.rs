use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::{Action, PageContent, PageTrait};

enum InputInvalid {
    Repeat,
    None,
    Empty,
}

pub struct CreatePhoneBookPage {
    page_content: PageContent,
    input_name: String,
    existed_list: Vec<String>,
    input_valid: InputInvalid,
}

impl CreatePhoneBookPage {
    pub fn new(existed_list: Vec<String>) -> CreatePhoneBookPage {
        let mut create_phone_book_page = CreatePhoneBookPage {
            page_content: PageContent::new(),
            input_name: "".to_string(),
            input_valid: InputInvalid::None,
            existed_list,
        };

        create_phone_book_page.refresh_content();

        create_phone_book_page
    }

    fn refresh_content(&mut self) {
        let mut page_content = PageContent::from_list(
            "Create Phone Book".to_string(),
            vec!["Name: ".to_string(), self.input_name.clone()],
            1,
            super::AlignType::Left,
        );

        if matches!(self.input_valid, InputInvalid::Repeat) {
            page_content.add_element(super::UiElement::Text(
                format!("{} has been existed.", self.input_name),
                crate::utils::AlignType::Left,
            ));
        } else if matches!(self.input_valid, InputInvalid::Empty) {
            page_content.add_element(super::UiElement::Text(
                format!("Phone book name cannot be empty."),
                crate::utils::AlignType::Left,
            ));
        }

        self.page_content = page_content;
    }
}

impl PageTrait for CreatePhoneBookPage {
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
                    self.input_name = format!("{}{}", self.input_name.clone(), ch);
                    self.input_valid = InputInvalid::None;
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    if self.input_name.len() >= 1 {
                        self.input_name.remove(self.input_name.len() - 1);
                    }
                    self.input_valid = InputInvalid::None;
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    if self.existed_list.contains(&self.input_name) {
                        self.input_valid = InputInvalid::Repeat
                    } else if self.input_name.is_empty() {
                        self.input_valid = InputInvalid::Empty;
                    } else {
                        action = Action::CreateNewPhoneBook(self.input_name.clone());
                    }
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => action = Action::Exit,
                _ => {}
            },
            _ => {}
        }

        // self.input_valid = input_invalid;

        self.refresh_content();

        action
    }

    fn render(&self) {
        self.page_content.render();
    }
}
