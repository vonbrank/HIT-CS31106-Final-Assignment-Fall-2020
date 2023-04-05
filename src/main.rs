use std::rc::Rc;

use screen::{
    canvas::{Canvas, RootViewNode},
    viewnode::{Border, BoxModelAttribute, LayoutType, ViewNode, ViewNodeType},
};

mod screen;
mod ui;

fn main() {
    let mut canvas = Canvas::new(60, 20);

    let text_view = Rc::new(ViewNode {
        box_model_attribute: BoxModelAttribute {
            padding: None,
            margin: None,
            border: None,
            width: None,
            height: None,
        },
        node_type: ViewNodeType::ContentType(screen::viewnode::ContentType::Text(
            "Hello world!".to_string(),
        )),
        child_nodes: vec![],
    });

    let box_view = ViewNode {
        box_model_attribute: BoxModelAttribute {
            padding: None,
            margin: None,
            border: Some(Border {
                left: 1,
                top: 1,
                right: 1,
                bottom: 1,
            }),
            width: Some(40),
            height: Some(15),
        },
        node_type: ViewNodeType::LayoutType(LayoutType::Box),
        child_nodes: vec![Rc::clone(&text_view)],
    };

    let root_view_node = RootViewNode(box_view);

    canvas.render_view_node_tree(&root_view_node);

    canvas.draw_on_screen();
}
