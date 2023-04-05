mod screen;
mod ui;
use crate::screen::canvas::Canvas;
use crate::ui::component::Block::BlockView;

fn main() {
    let mut root_view = BlockView::new(60, 20);

    let mut canvas = Canvas::new(60, 20);

    canvas.render_view(&mut root_view);

    canvas.draw_on_screen();
}
