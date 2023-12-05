use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use super::{handle_vertical_scroll, Action, PageContent, PageTrait};

lazy_static! {
    static ref HOME_ENTRY_ITEMS: Mutex<Vec<(String, HomeEntryAction)>> = Mutex::new(vec![
        ("New Phone Book".to_string(), HomeEntryAction::NewPhoneBook),
        (
            "Phone Book List".to_string(),
            HomeEntryAction::LoadPhoneBooks
        ),
        ("Settings".to_string(), HomeEntryAction::Settings),
        ("About".to_string(), HomeEntryAction::About),
        ("Exit".to_string(), HomeEntryAction::Exit),
    ]);
}

pub struct HomeEntry {
    page_content: PageContent,
    current_select_index: usize,
    home_entry_items: Vec<(String, HomeEntryAction)>,
}

impl HomeEntry {
    pub async fn new() -> HomeEntry {
        let mut home_entry = HomeEntry {
            page_content: PageContent::new(),
            current_select_index: 0,
            home_entry_items: HOME_ENTRY_ITEMS.lock().await.clone(),
        };

        home_entry.refresh_content();

        home_entry
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_list(
            "Contactify".to_string(),
            self.home_entry_items
                .iter()
                .map(|item| item.0.clone())
                .collect(),
            self.current_select_index,
            super::AlignType::Center,
        );
    }
}

impl PageTrait for HomeEntry {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        if handle_vertical_scroll(
            key_event,
            &self.home_entry_items,
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
                        let home_entry_action = self
                            .home_entry_items
                            .get(self.current_select_index)
                            .unwrap()
                            .1;
                        action = match home_entry_action {
                            HomeEntryAction::NewPhoneBook => {
                                Action::Navigate(super::PageType::NewPhoneBook)
                            }
                            HomeEntryAction::LoadPhoneBooks => {
                                Action::Navigate(super::PageType::PhoneBookList)
                            }
                            HomeEntryAction::Settings => {
                                Action::Navigate(super::PageType::Settings)
                            }
                            HomeEntryAction::About => Action::Navigate(super::PageType::About),
                            HomeEntryAction::Exit => Action::Exit,
                            _ => Action::None,
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

#[derive(Clone, Copy)]
pub enum HomeEntryAction {
    NewPhoneBook,
    LoadPhoneBooks,
    Settings,
    About,
    Exit,
}
