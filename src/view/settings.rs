use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::SettingsState;

use super::{
    handle_horizontal_scroll, handle_vertical_scroll, Action, PageContent, PageTrait, UiElement,
};

pub struct SettingsPage {
    page_content: PageContent,
    current_select_index: usize,
    setting_entries: Vec<(String, String)>,
    string_item_state_entries: Vec<(String, usize, Vec<String>)>,
    settings: SettingsState,
}
#[derive(Clone)]
pub struct SettingsPageSaved {
    current_select_index: usize,
}

impl SettingsPage {
    pub fn new(settings: SettingsState) -> SettingsPage {
        let mut settings_page = SettingsPage {
            page_content: PageContent::new(),
            current_select_index: 0,
            setting_entries: settings.string_entries(),
            string_item_state_entries: settings.string_item_state_entries(),
            settings,
        };

        settings_page.refresh_content();

        settings_page
    }

    pub fn restore(settings: SettingsState, saved: SettingsPageSaved) -> SettingsPage {
        let mut settings_page = SettingsPage {
            page_content: PageContent::new(),
            current_select_index: saved.current_select_index,
            setting_entries: settings.string_entries(),
            string_item_state_entries: settings.string_item_state_entries(),
            settings,
        };

        settings_page.refresh_content();

        settings_page
    }

    fn refresh_content(&mut self) {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text(
            "Settings".to_string(),
            super::AlignType::Center,
        ));
        page_content.add_element(UiElement::Text("-".repeat(128), super::AlignType::Left));
        page_content.add_element(UiElement::KeyValueList(
            self.setting_entries.clone(),
            self.current_select_index,
            super::AlignType::Left,
        ));
        self.page_content = page_content;
    }

    fn saved(&self) -> SettingsPageSaved {
        SettingsPageSaved {
            current_select_index: self.current_select_index,
        }
    }
}

impl PageTrait for SettingsPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        // let current_index = self.

        let current_select_entry = &mut self
            .string_item_state_entries
            .get(self.current_select_index)
            .unwrap();
        let mut new_select_entry_index = current_select_entry.1;

        if handle_vertical_scroll(
            key_event,
            &self.setting_entries,
            &mut self.current_select_index,
        ) {
            self.refresh_content();
        } else if handle_horizontal_scroll(
            key_event,
            &current_select_entry.2,
            &mut new_select_entry_index,
        ) {
            self.settings.update_item_select(
                self.setting_entries
                    .get(self.current_select_index)
                    .unwrap()
                    .0
                    .clone(),
                new_select_entry_index,
            );
            action = Action::UpdateSettings(self.settings.clone(), self.saved());
        } else {
            match key_event {
                KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                } => match key_event {
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
