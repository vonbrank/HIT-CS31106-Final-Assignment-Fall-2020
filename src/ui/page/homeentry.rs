use crate::{
    screen::viewnode::Padding,
    ui::{
        components::{text::Text, Component},
        layout::{column::Column, page::Page},
    },
};

pub struct HomeEntryState {
    pub currentOption: HomeEntryOption,
}

impl HomeEntryState {
    pub fn new() -> HomeEntryState {
        HomeEntryState {
            currentOption: HomeEntryOption::CreateNewContactsBook,
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
            format!("+{:^19}+", option_label)
        } else {
            format!(" {:^19} ", option_label)
        }
    }

    pub fn render(&self) -> Box<dyn Component> {
        let selected_label = Self::get_option_label(self.currentOption);

        let mut container = Column::new(vec![
            Box::new(Text::new(Self::hightlight_option_label_if_selected(
                Self::get_option_label(HomeEntryOption::CreateNewContactsBook),
                selected_label.clone(),
            ))),
            Box::new(Text::new(Self::hightlight_option_label_if_selected(
                Self::get_option_label(HomeEntryOption::ContactsBookList),
                selected_label.clone(),
            ))),
            Box::new(Text::new(Self::hightlight_option_label_if_selected(
                Self::get_option_label(HomeEntryOption::Settings),
                selected_label.clone(),
            ))),
            Box::new(Text::new(Self::hightlight_option_label_if_selected(
                Self::get_option_label(HomeEntryOption::About),
                selected_label.clone(),
            ))),
        ]);
        container.resize(58, 5).padding(Padding::new(0, 0, 1, 0));

        let page = Page::new("Contactify".to_string(), Box::new(container));
        Box::new(page)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HomeEntryOption {
    CreateNewContactsBook,
    ContactsBookList,
    Settings,
    About,
}
