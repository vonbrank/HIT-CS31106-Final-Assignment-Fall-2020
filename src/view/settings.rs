use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Settings;

use super::{handle_list_scroll, Action, PageContent, PageTrait, UiElement};

pub struct SettingsPage {
    page_content: PageContent,
    current_select_index: usize,
    settings: Settings,
    setting_list: Vec<(String, String)>,
}

impl SettingsPage {
    pub fn new(settings: Settings) -> SettingsPage {
        let mut settings_page = SettingsPage {
            page_content: PageContent::new(),
            current_select_index: 0,
            setting_list: settings.setting_list(),
            settings,
        };

        settings_page.refresh_content();

        settings_page
    }

    fn refresh_content(&mut self) {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text("Settings".to_string()));
        page_content.add_element(UiElement::Text("----------".to_string()));
        page_content.add_element(UiElement::KeyValueList(
            self.setting_list.clone(),
            self.current_select_index,
        ));
        self.page_content = page_content;
    }
}

impl PageTrait for SettingsPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        if handle_list_scroll(
            key_event,
            &self.setting_list,
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
