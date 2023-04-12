use crate::screen::viewnode::{Border, BoxModelAttribute, Margin, Padding, ViewNode};

use super::Component;

pub struct Divider {
    length: usize,
    direction: DividerDirection,
    variant: DividerVariant,
}

impl Divider {
    pub fn new(length: usize, direction: DividerDirection, variant: DividerVariant) -> Divider {
        Divider {
            length,
            direction,
            variant,
        }
    }
}

impl Component for Divider {
    fn to_view_mut(&mut self) -> ViewNode {
        match self.direction {
            DividerDirection::Horizontal => ViewNode {
                box_model_attribute: BoxModelAttribute::new(
                    self.length,
                    1,
                    Padding::new(0, 0, 0, 0),
                    Margin::new(0, 0, 0, 0),
                    Border::new(0, 1, 0, 0),
                ),
                node_type: ViewNode::box_layout(),
                child_nodes: vec![],
            },
            DividerDirection::Vertical => ViewNode {
                box_model_attribute: BoxModelAttribute::new(
                    1,
                    self.length,
                    Padding::new(0, 0, 0, 0),
                    Margin::new(0, 0, 0, 0),
                    Border::new(1, 0, 0, 0),
                ),
                node_type: ViewNode::box_layout(),
                child_nodes: vec![],
            },
        }
    }
}

pub enum DividerDirection {
    Horizontal,
    Vertical,
}

pub enum DividerVariant {
    Full,
    Dotted,
}
