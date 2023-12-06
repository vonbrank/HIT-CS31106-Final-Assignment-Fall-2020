use tokio::{fs, sync::mpsc};
use tokio::{join, task};

use crossterm::event::{self};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::model::{Model, PhoneBook, SettingsState};
use crate::{
    controller::{Controller, UpdateResult},
    view::PageContent,
};

#[derive(Serialize, Deserialize)]
pub struct SavingData {
    pub phone_books: Vec<PhoneBook>,
    pub settings: SettingsState,
}

impl SavingData {
    pub fn new(phone_books: Vec<PhoneBook>, settings: SettingsState) -> SavingData {
        SavingData {
            phone_books,
            settings,
        }
    }

    pub fn from_model(model: &Model) -> SavingData {
        SavingData {
            phone_books: model.phone_books.clone(),
            settings: model.settings.clone(),
        }
    }
}

pub struct App {
    controller: Option<Controller>,
}

impl App {
    pub fn new() -> App {
        App { controller: None }
    }

    async fn init(&mut self) {
        let (keyboard_event_sender, keyboard_event_receiver) = mpsc::unbounded_channel();

        let (saving_data_sender, mut saving_data_receiver) =
            mpsc::unbounded_channel::<SavingData>();

        task::spawn(async move {
            while let Ok(event) = event::read() {
                if let event::Event::Key(key_event) = event {
                    if let Err(_) = keyboard_event_sender.send(key_event) {
                        break;
                    }
                }
            }
        });

        let app_data_path = env!("APPDATA");
        let app_folder_name = "Contactify";
        let app_data_folder_path = format!("{}/{}", app_data_path, app_folder_name);
        fs::create_dir_all(&app_data_folder_path)
            .await
            .expect("failed to create dir");
        let phone_book_data_path = format!("{}/phone_book.json", &app_data_folder_path);
        let settings_data_path = format!("{}/settings.json", &app_data_folder_path);

        let phone_book_data_path1 = phone_book_data_path.clone();
        let settings_data_path1 = settings_data_path.clone();
        task::spawn(async move {
            let phone_book_data_path = phone_book_data_path1;
            let settings_data_path = settings_data_path1;

            while let Some(saving_data) = saving_data_receiver.recv().await {
                let phone_book_json =
                    serde_json::to_string_pretty(&saving_data.phone_books).unwrap();
                let settings_json = serde_json::to_string_pretty(&saving_data.settings).unwrap();
                let _ = fs::write(&phone_book_data_path, phone_book_json).await;
                let _ = fs::write(&settings_data_path, settings_json).await;
            }
        });

        let read_phone_book = fs::read_to_string(phone_book_data_path);
        let read_settings = fs::read_to_string(settings_data_path);

        let (read_phone_book_result, read_settings_results) = join!(read_phone_book, read_settings);

        match (read_phone_book_result, read_settings_results) {
            (Ok(phone_book_json), Ok(settings_json)) => {
                let phone_book_result =
                    serde_json::from_str::<Vec<PhoneBook>>(phone_book_json.as_str());
                let settings_result = serde_json::from_str::<SettingsState>(settings_json.as_str());

                match (phone_book_result, settings_result) {
                    (Ok(phone_book), Ok(settings)) => {
                        // println!("from app data");
                        self.controller = Some(Controller::from_saved_data(
                            keyboard_event_receiver,
                            saving_data_sender,
                            phone_book,
                            settings,
                        ))
                    }
                    _ => {
                        self.controller =
                            Some(Controller::new(keyboard_event_receiver, saving_data_sender));
                    }
                }
            }
            _ => {
                self.controller =
                    Some(Controller::new(keyboard_event_receiver, saving_data_sender));
            }
        }

        // let app_data = "Hello world!";
        // fs::write(&app_data_file_path, app_data)
        //     .await
        //     .expect("failed to write data");
    }

    pub async fn run(&mut self) {
        self.init().await;

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
