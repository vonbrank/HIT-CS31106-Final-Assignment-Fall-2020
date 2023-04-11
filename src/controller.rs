use std::error::Error;

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use tokio::sync::mpsc;
use tokio::task;

use crate::{
    model::Model,
    screen::canvas::{Canvas, RootViewNode},
};
pub struct Controller {
    canvas: Canvas,
    model: Model,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            canvas: Canvas::new(60, 20),
            model: Model::new(),
        }
    }

    pub async fn init(&mut self) {
        self.render();
    }

    fn render(&mut self) {
        let view_node = self.model.state.home_entry_state.render().to_view_mut();

        let root_view_node = RootViewNode(view_node);

        self.canvas.render_view_node_tree(&root_view_node);

        self.canvas.draw_on_screen();
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        terminal::enable_raw_mode()?;
        let (sender, mut receiver) = mpsc::unbounded_channel::<KeyCode>();

        // let (shut_down_sender, shut_down_receiver) = channel::<u32>();

        let sender_handle = task::spawn(async move {
            while let Ok(key_event) = crossterm::event::read() {
                match key_event {
                    Event::Key(KeyEvent {
                        kind: KeyEventKind::Press,
                        code,
                        ..
                    }) => {
                        if let Err(_) = sender.send(code) {
                            break;
                        }
                    }
                    _ => {}
                };
            }
        });

        loop {
            if let Some(key_code) = receiver.recv().await {
                if let KeyCode::Esc = key_code {
                    terminal::disable_raw_mode()?;
                    break;
                }
                self.control_home_page(key_code);
                self.render();
            } else {
                break;
            }
        }

        drop(receiver);

        println!("before abort...");

        sender_handle.abort();
        sender_handle.await?;

        Ok(())
    }

    fn control_home_page(&mut self, key_code: KeyCode) {
        let home_state = &mut self.model.state.home_entry_state;

        match key_code {
            KeyCode::Up => {
                home_state.to_previous_item();
            }
            KeyCode::Down => {
                home_state.to_next_item();
            }
            _ => {}
        }
    }
}
