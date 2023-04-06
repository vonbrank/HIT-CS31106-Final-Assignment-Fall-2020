use crate::screen::viewnode::{BoxModelAttribute, ViewNode};

use super::Component;

pub struct Text {
    string: String,
}

impl Text {
    pub fn new(string: String) -> Text {
        Text { string }
    }
}

impl Component for Text {
    fn to_view_node(&self) -> ViewNode {
        ViewNode {
            box_model_attribute: BoxModelAttribute::with_none(),
            node_type: ViewNode::text_node(self.string.clone()),
            child_nodes: vec![],
        }
    }
}
