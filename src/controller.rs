use crossterm::event::KeyEvent;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Controller {
    receiver: UnboundedReceiver<KeyEvent>,
}

impl Controller {
    pub fn new(receiver: UnboundedReceiver<KeyEvent>) -> Controller {
        Controller { receiver }
    }

    pub async fn update(&mut self) {
        self.handle_input().await;
    }

    async fn handle_input(&mut self) {
        if let Some(key_event) = self.receiver.recv().await {
            println!("{:?}", key_event);
        }
    }
}
