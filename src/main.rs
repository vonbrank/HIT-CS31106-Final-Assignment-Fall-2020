mod app;
mod controller;
mod model;
mod utils;
mod view;

use std::error::Error;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    app.run().await;

    Ok(())
}
