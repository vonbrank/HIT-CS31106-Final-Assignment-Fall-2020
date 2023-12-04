use crossterm::event::KeyEvent;

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
    pub fn create_page(&self) -> Box<dyn PageTrait> {
        match self {
            PageType::HomeEntry => Box::new(HomeEntry::new()),
            _ => Box::new(EmptyPage {}),
        }
    }
}

pub enum UiElement {
    Text(String),
    List(Vec<String>),
    KeyValue(Vec<(String, String)>),
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
                UiElement::List(list) => {
                    for item in list.into_iter() {
                        println!("{}", item);
                    }
                }
                UiElement::KeyValue(entries) => {
                    for (key, value) in entries.into_iter() {
                        println!("{} {}", key, value);
                    }
                }
            }
        }
    }
}

pub enum Action {
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

pub struct HomeEntry {
    page_content: PageContent,
}

impl HomeEntry {
    pub fn new() -> HomeEntry {
        let mut page_content = PageContent::new();
        page_content.add_element(UiElement::Text("Contactify".to_string()));
        page_content.add_element(UiElement::Text("----------".to_string()));
        page_content.add_element(UiElement::List(vec![
            "New Phone Book".to_string(),
            "Phone Book List".to_string(),
            "Settings".to_string(),
            "About".to_string(),
            "Exit".to_string(),
        ]));

        HomeEntry {
            page_content: page_content,
        }
    }
}

impl PageTrait for HomeEntry {
    fn handle_input(&mut self, key_event: KeyEvent) -> Action {
        Action::None
    }

    fn render(&self) {
        self.page_content.render();
    }
}
