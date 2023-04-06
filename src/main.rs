use std::rc::Rc;

use model::MODEL;
use screen::{
    canvas::{Canvas, RootViewNode},
    viewnode::{
        Border, BoxModelAttribute, BoxModelQuadruple, LayoutType, Margin, Padding, ViewNode,
        ViewNodeType,
    },
};
use ui::{
    components::{divider::Divider, text::Text, Component},
    layout::{column::Column, page::Page, row::Row},
};

mod model;
mod screen;
mod ui;

fn main() {
    let mut canvas = Canvas::new(60, 20);

    {
        let model = MODEL.lock().unwrap();

        let root_view_node = RootViewNode(
            model
                .borrow()
                .state
                .home_entry_state
                .render()
                .to_view_node(),
        );

        canvas.render_view_node_tree(&root_view_node);

        canvas.draw_on_screen();
    }
}
