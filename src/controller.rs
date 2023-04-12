use std::error::Error;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use tokio::sync::mpsc;
use tokio::task;

use crate::{
    model::{contactmodel::Contact, Model, Page},
    screen::canvas::{Canvas, RootViewNode},
    ui::page::homeentry::HomeEntryOption,
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
        let contact_list_page_state = &mut self.model.state.contact_list_page_state;
        contact_list_page_state.name = "Contact List 1".to_string();
        contact_list_page_state.contact_list = vec![
            Contact {
                name: "Alice".to_string(),
                telephone: "123456".to_string(),
            },
            Contact {
                name: "Bob".to_string(),
                telephone: "567890".to_string(),
            },
        ];

        self.render();
    }

    fn render(&mut self) {
        let view_node = match self.model.current_page {
            Page::HomeEntry => self.model.state.home_entry_state.render().to_view_mut(),
            Page::ContactsBookDetail => self
                .model
                .state
                .contact_list_page_state
                .render()
                .to_view_mut(),
            _ => self.model.state.home_entry_state.render().to_view_mut(),
        };

        let root_view_node = RootViewNode(view_node);

        self.canvas.render_view_node_tree(&root_view_node);

        self.canvas.draw_on_screen();
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
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

        loop {
            if let Some(key_event) = rx.recv().await {
                match (key_event, self.model.current_page) {
                    (
                        KeyEvent {
                            kind: KeyEventKind::Press,
                            code: KeyCode::Esc, ..
                        },
                        Page::HomeEntry,
                    ) => {
                        break;
                    }
                    _ => {}
                }
                match key_event {
                    KeyEvent {
                        kind: KeyEventKind::Press,
                        code,
                        ..
                    } => {
                        match self.model.current_page {
                            Page::HomeEntry => {
                                self.control_home_page(code);
                            }
                            Page::ContactsBookDetail => {
                                self.control_contacts_book_detail(code);
                            }
                            _ => {}
                        }

                        self.render();
                    }
                    _ => {}
                };
            }
        }

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
            KeyCode::Enter => {
                let option = home_state.option_list[home_state.current_option_index];
                match option {
                    HomeEntryOption::CreateNewContactsBook => {
                        self.model.current_page = Page::ContactsBookDetail;
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    fn control_contacts_book_detail(&mut self, key_code: KeyCode) {
        let contacts_book_detail_state = &mut self.model.state.contact_list_page_state;
        match key_code {
            KeyCode::Up => {
                contacts_book_detail_state.to_previous_item();
            }
            KeyCode::Down => {
                contacts_book_detail_state.to_next_item();
            }
            KeyCode::Esc => self.model.current_page = Page::HomeEntry,
            _ => {}
        }
    }
}
