use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    model::Model,
    view::{Action, HomeEntry, HomeEntryAction, PageTrait, PageType},
};

pub struct Controller {
    receiver: UnboundedReceiver<KeyEvent>,
    current_page_type: PageType,
    page_cache: HashMap<PageType, Box<dyn PageTrait>>,
    model: Model,
}

impl Controller {
    pub fn new(receiver: UnboundedReceiver<KeyEvent>) -> Controller {
        Controller {
            receiver,
            current_page_type: PageType::HomeEntry,
            page_cache: HashMap::new(),
            model: Model::from_fake_data(),
        }
    }

    pub async fn update(&mut self) -> UpdateResult {
        let mut update_result = UpdateResult::Continue;

        let action = self.handle_input().await;

        match action {
            Action::HomeEntry(home_entry_action) => match home_entry_action {
                HomeEntryAction::LoadPhoneBooks => self.current_page_type = PageType::PhoneBookList,
                _ => {}
            },
            Action::Exit => match self.current_page_type {
                PageType::HomeEntry => update_result = UpdateResult::Exit,
                PageType::PhoneBookList => self.current_page_type = PageType::HomeEntry,
                _ => {}
            },
            _ => {}
        }

        self.render().await;

        update_result
    }

    async fn handle_input(&mut self) -> Action {
        if let Some(key_event) = self.receiver.recv().await {
            let current_page_view = self.page_cache.get_mut(&self.current_page_type).unwrap();
            current_page_view.handle_input(key_event)
        } else {
            Action::None
        }
    }

    pub async fn render(&mut self) {
        print!("\x1B[2J\x1B[1;1H");

        let current_page_view = self.page_cache.get(&self.current_page_type);
        if let Some(page_view) = current_page_view {
            page_view.render();
        } else {
            let page_view = self
                .current_page_type
                .create_page_from_model(&self.model)
                .await;
            page_view.render();
            self.page_cache
                .insert(self.current_page_type.clone(), page_view);
        }
    }

    pub fn close(&mut self) {
        self.receiver.close();
    }
}

pub enum UpdateResult {
    Continue,
    Exit,
}
