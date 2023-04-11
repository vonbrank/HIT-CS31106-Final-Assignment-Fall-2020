use crate::{
    screen::viewnode::Padding,
    ui::{
        components::{text::Text, Component},
        layout::{column::Column, page::Page},
    },
};

pub struct HomeEntryState {
    pub current_option_index: usize,
    pub option_list: Vec<HomeEntryOption>,
}

impl HomeEntryState {
    pub fn new() -> HomeEntryState {
        HomeEntryState {
            current_option_index: 0,
            option_list: vec![
                HomeEntryOption::CreateNewContactsBook,
                HomeEntryOption::ContactsBookList,
                HomeEntryOption::Settings,
                HomeEntryOption::About,
            ],
        }
    }

    pub fn get_option_label(option: HomeEntryOption) -> String {
        match option {
            HomeEntryOption::CreateNewContactsBook => "New contacts book".to_string(),
            HomeEntryOption::ContactsBookList => "Contacts books list".to_string(),
            HomeEntryOption::Settings => "Settings".to_string(),
            HomeEntryOption::About => "About".to_string(),
        }
    }

    fn hightlight_option_label_if_selected(option_label: String, selected_label: String) -> String {
        if option_label == selected_label {
            format!("+{:<19}+", option_label)
        } else {
            format!(" {:<19} ", option_label)
        }
    }

    pub fn render(&self) -> Box<dyn Component> {
        let selected_label = Self::get_option_label(self.option_list[self.current_option_index]);

        let mut option_component_list: Vec<Box<dyn Component>> = vec![];

        for option_item in self.option_list.iter() {
            let item_component = Box::new(Text::new(Self::hightlight_option_label_if_selected(
                Self::get_option_label(*option_item),
                selected_label.clone(),
            )));
            option_component_list.push(item_component);
        }

        let mut container = Column::new(option_component_list);
        container.resize(58, 5).padding(Padding::new(0, 0, 1, 0));

        let page = Page::new("Contactify".to_string(), Box::new(container));
        Box::new(page)
    }

    pub fn to_next_item(&mut self) {
        let max_index = self.option_list.len() - 1;
        if self.current_option_index + 1 <= max_index {
            self.current_option_index += 1;
        }
    }

    pub fn to_previous_item(&mut self) {
        if self.current_option_index >= 1 {
            self.current_option_index -= 1;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HomeEntryOption {
    CreateNewContactsBook,
    ContactsBookList,
    Settings,
    About,
}
