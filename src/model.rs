use std::{cell::RefCell, sync::Mutex};

use lazy_static::lazy_static;

use crate::ui::page::homeentry::HomeEntryState;

pub struct Model {
    pub current_page: Page,
    pub state: State,
    pub settings: Settings,
}

impl Model {
    pub fn new() -> Model {
        Model {
            current_page: Page::HomeEntry,
            state: State {
                home_entry_state: HomeEntryState::new(),
            },
            settings: Settings {},
        }
    }
}

lazy_static! {
    pub static ref MODEL: Mutex<RefCell<Model>> = Mutex::new(RefCell::new(Model::new()));
}

pub struct State {
    pub home_entry_state: HomeEntryState,
}

pub enum Page {
    HomeEntry,
    Settings,
    CreateContactsBook,
    ContactsBookList,
    ContactsBookDetail,
    CreateContact,
}

pub struct Settings {}
