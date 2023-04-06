use std::rc::Rc;

use crate::{
    screen::viewnode::{Border, BoxModelAttribute, Margin, Padding, ViewNode},
    ui::components::Component,
};

pub struct Row {
    box_model_attribute: BoxModelAttribute,
    child_components: Vec<Box<dyn Component>>,
}

impl Row {
    pub fn new(child_components: Vec<Box<dyn Component>>) -> Self {
        Row {
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

impl Component for Row {
    fn to_view_node(&self) -> ViewNode {
        let mut child_nodes = Vec::new();

        self.child_components.iter().for_each(|child_component| {
            child_nodes.push(Rc::new(child_component.to_view_node()));
        });

        ViewNode {
            box_model_attribute: self.box_model_attribute,
            node_type: ViewNode::row_layout(),
            child_nodes,
        }
    }
}
