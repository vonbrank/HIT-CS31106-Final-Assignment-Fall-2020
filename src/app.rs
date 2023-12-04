use tokio::sync::mpsc;
use tokio::task;

use crossterm::event::{self};

use crate::controller::Controller;

pub struct App {
    controller: Option<Controller>,
}

impl App {
    pub fn new() -> App {
        App { controller: None }
    }

    pub fn init(&mut self) {
        let (tx, mut rx) = mpsc::unbounded_channel();

        task::spawn(async move {
            while let Ok(event) = event::read() {
                if let event::Event::Key(key_event) = event {
                    if let Err(_) = tx.send(key_event) {
                        break;
                    }
                }
            }
        });

        self.controller = Some(Controller::new(rx));
    }

    pub async fn run(&mut self) {
        self.init();

        let controller = self.controller.as_mut().unwrap();

        controller.render();

        loop {
            controller.update().await;
        }
    }
}
