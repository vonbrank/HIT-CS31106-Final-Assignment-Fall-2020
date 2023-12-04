use std::usize;

use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use crate::model::Model;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PageType {
    HomeEntry,
    Settings,
    About,
    NewPhoneBook,
    PhoneBookList,
    PhoneBook(String),
    Contact(String, String),
}

impl PageType {
    pub async fn create_page(&self) -> Box<dyn PageTrait> {
        match self {
            PageType::HomeEntry => Box::new(HomeEntry::new().await),
            _ => Box::new(EmptyPage {}),
        }
    }

    pub async fn create_page_from_model(&self, model: &Model) -> Box<dyn PageTrait> {
        match self {
            PageType::HomeEntry => self.create_page().await,
            PageType::PhoneBookList => {
                let phone_book_list: Vec<String> = model
                    .phone_books
                    .iter()
                    .map(|item| item.name.clone())
                    .collect();
                let phone_book_list_page = PhoneBookListPage::new(phone_book_list);
                Box::new(phone_book_list_page)
            }
            _ => Box::new(EmptyPage {}),
        }
    }
}

pub enum UiElement {
    Text(String),
    KeyValue((String, String)),
    TextList(Vec<String>, usize),
    KeyValueList(Vec<(String, String)>, usize),
}

pub struct PageContent {
    ui_elements: Vec<UiElement>,
}

impl PageContent {
    pub fn new() -> PageContent {
        PageContent {
            ui_elements: vec![],
        }
    }
    pub fn from_list(name: String, list: Vec<String>, selected_index: usize) -> PageContent {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text(name));
        page_content.add_element(UiElement::Text("----------".to_string()));
        page_content.add_element(UiElement::TextList(list, selected_index));
        page_content
    }
    pub fn add_element(&mut self, element: UiElement) {
        self.ui_elements.push(element);
    }
    pub fn render(&self) {
        for ui_element in self.ui_elements.iter() {
            match ui_element {
                UiElement::Text(text) => {
                    println!("{}", text);
                }
                UiElement::KeyValue((key, value)) => {
                    println!("{} {}", key, value);
                }
                UiElement::TextList(list, select_index) => {
                    for (index, item) in list.into_iter().enumerate() {
                        if index == *select_index {
                            println!("\x1B[7m{}\x1B[0m", item);
                        } else {
                            println!("{}", item);
                        }
                    }
                }
                UiElement::KeyValueList(entries, select_index) => {
                    for (index, (key, value)) in entries.into_iter().enumerate() {
                        if index == *select_index {
                            println!("\x1B[7m{} {}\x1B[0m", key, value);
                        } else {
                            println!("{} {}", key, value);
                        }
                    }
                }
            }
        }
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

pub enum Action {
    HomeEntry(HomeEntryAction),
    Exit,
    None,
}

pub trait PageTrait {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action;
    fn render(&self);
}

pub struct EmptyPage {}

impl PageTrait for EmptyPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        Action::None
    }

    fn render(&self) {}
}

fn handle_list_scroll<T>(
    key_event: KeyEvent,
    target_list: &Vec<T>,
    seleced_index: &mut usize,
) -> bool {
    let mut res = false;

    match key_event {
        KeyEvent {
            kind: KeyEventKind::Press,
            ..
        } => match key_event {
            KeyEvent {
                code: KeyCode::Up, ..
            } => {
                if *seleced_index <= 0 {
                    *seleced_index = target_list.len() - 1;
                } else {
                    *seleced_index -= 1;
                }
                res = true;
            }
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => {
                if *seleced_index >= target_list.len() - 1 {
                    *seleced_index = 0;
                } else {
                    *seleced_index += 1;
                }
                res = true;
            }
            _ => {}
        },
        _ => {}
    }

    res
}

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
            "Phone Book List".to_string(),
            self.home_entry_items
                .iter()
                .map(|item| item.0.clone())
                .collect(),
            self.current_select_index,
        );
    }
}

impl PageTrait for HomeEntry {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        if handle_list_scroll(
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
                        action = Action::HomeEntry(home_entry_action);
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

pub struct PhoneBookListPage {
    page_content: PageContent,
    current_select_index: usize,
    phone_book_list: Vec<String>,
}

impl PhoneBookListPage {
    pub fn new(phone_book_list: Vec<String>) -> PhoneBookListPage {
        let mut phone_book_list_page = PhoneBookListPage {
            page_content: PageContent::new(),
            current_select_index: 0,
            phone_book_list,
        };

        phone_book_list_page.refresh_content();

        phone_book_list_page
    }

    fn refresh_content(&mut self) {
        self.page_content = PageContent::from_list(
            "Phone Book List".to_string(),
            self.phone_book_list.clone(),
            self.current_select_index,
        );
    }
}
#[async_trait]
impl PageTrait for PhoneBookListPage {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        if handle_list_scroll(
            key_event,
            &self.phone_book_list,
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
