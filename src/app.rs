use tokio::sync::mpsc;
use tokio::task;

use crossterm::event::{self};

use crate::{
    controller::{Controller, UpdateResult},
    view::PageContent,
};

pub struct App {
    controller: Option<Controller>,
}

impl App {
    pub fn new() -> App {
        App { controller: None }
    }

    fn init(&mut self) {
        let (tx, mut rx) = mpsc::unbounded_channel();

        task::spawn(async move {
            while let Ok(event) = event::read() {
                if let event::Event::Key(key_event) = event {
                    if let Err(_) = tx.send(key_event) {
                        break;
                    }
                }
            }
            println!("Receiver has been closed.");
        });

        self.controller = Some(Controller::new(rx));
    }

    pub async fn run(&mut self) {
        self.init();

        let controller = self.controller.as_mut().unwrap();

        controller.render().await;

        loop {
            let update_result = controller.update().await;

            if matches!(update_result, UpdateResult::Exit) {
                break;
            }
        }

        controller.close();
    }
}
