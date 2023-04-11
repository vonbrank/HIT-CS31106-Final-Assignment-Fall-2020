use std::error::Error;

use controller::Controller;

mod controller;
mod model;
mod screen;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app_controller = Controller::new();
    app_controller.init().await;
    app_controller.run().await?;

    Ok(())
}
