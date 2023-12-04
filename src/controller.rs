use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::view::{Action, PageTrait, PageType};

pub struct Controller {
    receiver: UnboundedReceiver<KeyEvent>,
    current_page_type: PageType,
    view_table: HashMap<PageType, Box<dyn PageTrait>>,
}

impl Controller {
    pub fn new(receiver: UnboundedReceiver<KeyEvent>) -> Controller {
        Controller {
            receiver,
            current_page_type: PageType::HomeEntry,
            view_table: HashMap::new(),
        }
    }

    pub async fn update(&mut self) {
        self.handle_input().await;

        self.render();
    }

    async fn handle_input(&mut self) -> Action {
        if let Some(key_event) = self.receiver.recv().await {
            let current_page_view = self.view_table.get_mut(&self.current_page_type).unwrap();
            current_page_view.handle_input(key_event)
        } else {
            Action::None
        }
    }

    pub fn render(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
        let current_page_view = self.view_table.get(&self.current_page_type);
        if let Some(page_view) = current_page_view {
            page_view.render();
        } else {
            let page_view = self.current_page_type.create_page();
            page_view.render();
            self.view_table
                .insert(self.current_page_type.clone(), page_view);
        }
    }
}
