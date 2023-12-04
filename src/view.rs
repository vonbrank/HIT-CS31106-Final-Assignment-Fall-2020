use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

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

#[async_trait]
pub trait PageTrait {
    async fn handle_input(&mut self, key_event: KeyEvent) -> Action;
    fn render(&self);
}

pub struct EmptyPage {}

#[async_trait]
impl PageTrait for EmptyPage {
    async fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        Action::None
    }

    fn render(&self) {}
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
}

impl HomeEntry {
    pub async fn new() -> HomeEntry {
        let mut home_entry = HomeEntry {
            page_content: PageContent::new(),
            current_select_index: 0,
        };

        home_entry.refresh_content().await;

        home_entry
    }

    async fn refresh_content(&mut self) {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text("Contactify".to_string()));
        page_content.add_element(UiElement::Text("----------".to_string()));
        page_content.add_element(UiElement::TextList(
            HOME_ENTRY_ITEMS
                .lock()
                .await
                .iter()
                .map(|item| item.0.clone())
                .collect(),
            self.current_select_index,
        ));
        self.page_content = page_content;
    }
}

#[async_trait]
impl PageTrait for HomeEntry {
    async fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        let mut action = Action::None;

        match key_event {
            KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            } => {
                if self.current_select_index <= 0 {
                    self.current_select_index = HOME_ENTRY_ITEMS.lock().await.len() - 1;
                } else {
                    self.current_select_index -= 1;
                }
            }
            KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            } => {
                if self.current_select_index >= HOME_ENTRY_ITEMS.lock().await.len() - 1 {
                    self.current_select_index = 0;
                } else {
                    self.current_select_index += 1;
                }
            }
            _ => {}
        }

        self.refresh_content().await;

        action
    }

    fn render(&self) {
        self.page_content.render();
    }
}
