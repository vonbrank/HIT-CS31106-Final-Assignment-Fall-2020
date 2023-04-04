mod renderer;
mod ui;
use crate::renderer::canvas::GLOBAL_CANVAS;
use crate::ui::view::view_base::ViewBase;

use crate::ui::view::View;

fn main() {
    println!("Hello, world!");
    let view = View {};
    view.draw();
    let global_canvas = GLOBAL_CANVAS.lock().unwrap();
    let global_canvas = global_canvas.borrow();
    global_canvas.draw();
}
