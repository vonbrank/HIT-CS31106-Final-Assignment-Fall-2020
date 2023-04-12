use crate::{
    model::contactmodel::Contact,
    screen::viewnode::Padding,
    ui::{
        components::{text::Text, Component},
        layout::{column::Column, page::Page},
    },
};

pub struct ContactListPageState {
    contact_list: Vec<Contact>,
    current_tab_index: usize,
    number_of_item_per_tab: usize,
    current_selected_index: usize,
    pub name: String,
    contact_display_list: Vec<Contact>,
}

impl ContactListPageState {
    pub fn new(
        name: String,
        contact_list: Vec<Contact>,
        current_tab_index: usize,
        number_of_item_per_tab: usize,
    ) -> ContactListPageState {
        ContactListPageState {
            contact_list,
            current_tab_index,
            number_of_item_per_tab,
            name,
            current_selected_index: 0,
            contact_display_list: vec![],
        }
    }

    pub fn render(&self) -> Box<dyn Component> {
        let mut option_component_list: Vec<Box<dyn Component>> = vec![];

        let max_name_length = 15;
        // contact_display_list.iter().for_each(|contact| {
        //     if max_name_length < contact.name.len() {
        //         max_name_length = contact.name.len();
        //     }
        // });

        for i in 0..self.contact_display_list.len() {
            let contact = &self.contact_display_list[i];
            let presuffix = if i == self.current_selected_index {
                "+"
            } else {
                " "
            };
            option_component_list.push(Box::new(Text::new(format!(
                "{}{}{}{}| {}",
                presuffix.to_string(),
                contact.name,
                " ".to_string().repeat(max_name_length - contact.name.len()),
                presuffix.to_string(),
                contact.telephone
            ))));
        }

        let mut container = Column::new(option_component_list);
        container.resize(58, 5).padding(Padding::new(0, 0, 1, 0));

        let page = Page::new(self.name.clone(), Box::new(container));
        Box::new(page)
    }

    pub fn to_next_item(&mut self) {
        if self.current_selected_index + 1 <= self.number_of_item_per_tab
            && self.current_selected_index + 1 < self.contact_display_list.len()
        {
            self.current_selected_index += 1;
        }
        self.update_contact_list_to_display();
    }

    pub fn to_previous_item(&mut self) {
        if self.current_selected_index >= 1 {
            self.current_selected_index -= 1;
        }
        self.update_contact_list_to_display();
    }

    fn reset_index(&mut self) {
        self.current_tab_index = 0;
        self.current_selected_index = 0;
        self.update_contact_list_to_display();
    }

    pub fn set_contact_list(&mut self, new_list: Vec<Contact>) {
        self.contact_list = new_list;
        self.reset_index();
    }

    fn update_contact_list_to_display(&mut self) {
        self.contact_display_list = vec![];
        for i in (self.current_tab_index * self.number_of_item_per_tab)
            ..(self.current_selected_index + 1) * self.number_of_item_per_tab
        {
            if i < self.contact_list.len() {
                let contact = self.contact_list[i].clone();
                self.contact_display_list.push(contact);
            }
        }
    }
}
