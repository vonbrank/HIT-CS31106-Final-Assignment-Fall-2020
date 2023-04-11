use std::rc::Rc;

use crate::screen::viewnode::{Border, BoxModelAttribute, Margin, Padding, ViewNode};
use crate::ui::components::Component;

pub struct Column {
    box_model_attribute: BoxModelAttribute,
    child_components: Vec<Box<dyn Component>>,
}

impl Column {
    pub fn new(child_components: Vec<Box<dyn Component>>) -> Self {
        Column {
            box_model_attribute: BoxModelAttribute::with_none(),
            child_components,
        }
    }

    pub fn border(&mut self, border: Border) -> &mut Self {
        self.box_model_attribute.border = Some(border);
        self
    }
    pub fn padding(&mut self, padding: Padding) -> &mut Self {
        self.box_model_attribute.padding = Some(padding);
        self
    }
    pub fn margin(&mut self, margin: Margin) -> &mut Self {
        self.box_model_attribute.margin = Some(margin);
        self
    }

    pub fn resize(&mut self, width: usize, height: usize) -> &mut Self {
        self.box_model_attribute.width = Some(width);
        self.box_model_attribute.height = Some(height);
        self
    }
}

impl Component for Column {
    fn to_view_mut(&mut self) -> ViewNode {
        let mut child_nodes = Vec::new();

        self.child_components
            .iter_mut()
            .for_each(|child_component| {
                child_nodes.push(Rc::new(child_component.to_view_mut()));
            });

        ViewNode {
            box_model_attribute: self.box_model_attribute,
            node_type: ViewNode::column_layout(),
            child_nodes,
        }
    }
}
