use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use super::{Action, PageContent, PageTrait};

pub struct About {
    page_content: PageContent,
}

impl About {
    pub fn new() -> About {
        let mut about = About {
            page_content: PageContent::new(),
        };
        about.refresh_content();
        about
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_lines(
            "About".to_string(),
            vec![
                "Author: Von Brank".to_string(),
                "Harbin Institute of Technology".to_string(),
                "Copyright 2023".to_string(),
            ],
            super::AlignType::Center,
        );
    }
}

impl PageTrait for About {
    fn handle_input(&mut self, key_event: crossterm::event::KeyEvent) -> super::Action {
        let mut action = Action::None;
        match key_event {
            KeyEvent {
                kind: KeyEventKind::Press,
                ..
            } => match key_event {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => action = Action::Navigate(super::PageType::HomeEntry),
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
