use std::rc::Rc;

use screen::{
    canvas::{Canvas, RootViewNode},
    viewnode::{
        Border, BoxModelAttribute, BoxModelQuadruple, LayoutType, Margin, Padding, ViewNode,
        ViewNodeType,
    },
};

mod screen;
mod ui;

fn main() {
    let mut canvas = Canvas::new(60, 20);

    let text_view = Rc::new(ViewNode {
        box_model_attribute: BoxModelAttribute::with_none(),
        node_type: ViewNode::text_node(
            &"It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. ".to_string()
        ),
        child_nodes: vec![],
    });
    let text_view1 = Rc::new(ViewNode {
        box_model_attribute: BoxModelAttribute::with_none(),
        node_type: ViewNode::text_node(
            &"It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. ".to_string()
        ),
        child_nodes: vec![],
    });

    let divider = Rc::new(ViewNode {
        box_model_attribute: BoxModelAttribute::new(
            35,
            1,
            Padding::new(0, 0, 0, 0),
            Margin::new(0, 0, 0, 0),
            Border::new(0, 1, 0, 0),
        ),
        node_type: ViewNode::box_layout(),
        child_nodes: vec![],
    });

    let mut box_view = ViewNode {
        box_model_attribute: BoxModelAttribute::with_width_and_height(35, 15),
        node_type: ViewNode::column_layout(),
        child_nodes: vec![
            Rc::clone(&text_view),
            Rc::clone(&divider),
            Rc::clone(&text_view1),
        ],
    };
    box_view.box_model_attribute.border = Some(Border::new(1, 1, 1, 1));
    box_view.box_model_attribute.padding = Some(Padding::new(1, 0, 1, 1));

    let root_view_node = RootViewNode(box_view);

    canvas.render_view_node_tree(&root_view_node);

    canvas.draw_on_screen();
}
