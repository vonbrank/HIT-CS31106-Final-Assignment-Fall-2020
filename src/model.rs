use crate::ui::page::{contactlist::ContactListPageState, homeentry::HomeEntryState};
pub mod contactmodel;

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
                contact_list_page_state: ContactListPageState::new("".to_string(), vec![], 0, 5),
            },
            settings: Settings {},
        }
    }
}

pub struct State {
    pub home_entry_state: HomeEntryState,
    pub contact_list_page_state: ContactListPageState,
}

#[derive(Copy, Clone)]
pub enum Page {
    HomeEntry,
    Settings,
    CreateContactsBook,
    ContactsBookList,
    ContactsBookDetail,
    CreateContact,
}

pub struct Settings {}
