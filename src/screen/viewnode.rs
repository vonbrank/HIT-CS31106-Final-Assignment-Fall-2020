use std::rc::Rc;

pub struct ViewNode {
    pub box_model_attribute: BoxModelAttribute,
    pub node_type: ViewNodeType,

    pub child_nodes: Vec<Rc<ViewNode>>,
}

impl ViewNode {
    pub fn box_layout() -> ViewNodeType {
        ViewNodeType::LayoutType(LayoutType::Box)
    }
    pub fn column_layout() -> ViewNodeType {
        ViewNodeType::LayoutType(LayoutType::Column)
    }
    pub fn row_layout() -> ViewNodeType {
        ViewNodeType::LayoutType(LayoutType::Row)
    }
    pub fn text_node(text: &String) -> ViewNodeType {
        ViewNodeType::ContentType(ContentType::Text(text.clone()))
    }
}

pub struct BoxModelAttribute {
    pub padding: Option<Padding>,
    pub margin: Option<Margin>,
    pub border: Option<Border>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl BoxModelAttribute {
    pub fn new(
        width: usize,
        height: usize,
        padding: Padding,
        margin: Margin,
        border: Border,
    ) -> BoxModelAttribute {
        BoxModelAttribute {
            padding: Some(padding),
            margin: Some(margin),
            border: Some(border),
            width: Some(width),
            height: Some(height),
        }
    }

    pub fn with_width_and_height(width: usize, height: usize) -> BoxModelAttribute {
        BoxModelAttribute {
            padding: None,
            margin: None,
            border: None,
            width: Some(width),
            height: Some(height),
        }
    }

    pub fn with_none() -> BoxModelAttribute {
        BoxModelAttribute {
            padding: None,
            margin: None,
            border: None,
            width: None,
            height: None,
        }
    }
}

pub struct BoxModelQuadruple {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl BoxModelQuadruple {
    pub fn new(left: usize, top: usize, right: usize, bottom: usize) -> BoxModelQuadruple {
        BoxModelQuadruple {
            left,
            top,
            right,
            bottom,
        }
    }
}

pub type Border = BoxModelQuadruple;
pub type Margin = BoxModelQuadruple;
pub type Padding = BoxModelQuadruple;

#[derive(Debug)]
pub enum ViewNodeType {
    LayoutType(LayoutType),
    ContentType(ContentType),
}
#[derive(Debug)]
pub enum LayoutType {
    Column,
    Row,
    Box,
}
#[derive(Debug)]
pub enum ContentType {
    Text(String),
}
