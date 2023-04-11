use std::cell::RefCell;

use crate::{
    screen::viewnode::{Border, Padding},
    ui::{
        self,
        components::{divider::Divider, text::Text, Component},
    },
};

use super::column::Column;

pub struct Page {
    pub title: String,
    pub content: Option<Box<dyn Component>>,
}

impl Page {
    pub fn new(title: String, content: Box<dyn Component>) -> Page {
        Page {
            title,
            content: Some(content),
        }
    }
}

impl Component for Page {
    fn to_view_mut(&mut self) -> crate::screen::viewnode::ViewNode {
        let title = Text::new(self.title.clone());

        let mut top_bar = Column::new(vec![Box::new(title)]);
        top_bar.resize(58, 1).padding(Padding::new(1, 0, 1, 0));

        let divider = Divider::new(58, ui::components::divider::DividerDirection::Horizontal);

        let mut child_components: Vec<Box<dyn Component>> =
            vec![Box::new(top_bar), Box::new(divider)];

        if let Some(_) = &self.content {
            let content = self.content.take().unwrap();
            child_components.push(content);
        }

        let mut app = Column::new(child_components);
        app.border(Border::new(1, 1, 1, 1)).resize(60, 15);
        app.to_view_mut()
    }
}
