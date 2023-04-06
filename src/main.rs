use std::rc::Rc;

use screen::{
    canvas::{Canvas, RootViewNode},
    viewnode::{
        Border, BoxModelAttribute, BoxModelQuadruple, LayoutType, Margin, Padding, ViewNode,
        ViewNodeType,
    },
};
use ui::{
    components::{divider::Divider, text::Text, Component},
    layout::{column::Column, row::Row},
};

mod screen;
mod ui;

fn main() {
    let mut canvas = Canvas::new(60, 20);

    let title = Text::new("It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. ".to_string());

    let mut top_bar = Column::new(vec![Box::new(title)]);
    top_bar.resize(33, 1).padding(Padding::new(1, 0, 1, 0));

    let divider = Divider::new(33, ui::components::divider::DividerDirection::Horizontal);

    let content_text = Text::new("It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. ".to_string());

    let mut text_a_box = Row::new(vec![Box::new(Text::new("text-a".to_string()))]);
    text_a_box
        .resize("text-a".len() + 1, 1)
        .padding(Padding::new(0, 0, 1, 0));

    let mut row = Row::new(vec![
        Box::new(text_a_box),
        Box::new(Divider::new(
            1,
            ui::components::divider::DividerDirection::Vertical,
        )),
        Box::new(Text::new("text-b".to_string())),
    ]);
    row.resize(31, 5);

    let mut container = Column::new(vec![Box::new(content_text), Box::new(row)]);
    container.resize(33, 10).padding(Padding::new(1, 0, 1, 0));

    let mut app = Column::new(vec![
        Box::new(top_bar),
        Box::new(divider),
        Box::new(container),
    ]);
    app.border(Border::new(1, 1, 1, 1)).resize(35, 15);

    let root_view_node = RootViewNode(app.to_view_node());

    canvas.render_view_node_tree(&root_view_node);

    canvas.draw_on_screen();
}
