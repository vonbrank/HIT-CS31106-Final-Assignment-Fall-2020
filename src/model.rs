#[derive(Clone)]
pub struct Contact {
    name: String,
    phone_number: String,
}

#[derive(Clone)]
pub struct PhoneBook {
    pub name: String,
    pub contacts: Vec<Contact>,
}

pub enum ResolutionOption {
    R40_10,
    R60_15,
    R80_20,
}

pub enum SettingOption {
    Resolution(String, i32),
    RefreshRate(String, i32),
    VSync(String, i32),
    Language(String, i32),
    DeveloperMode(String, i32),
}

pub struct Model {
    pub phone_books: Vec<PhoneBook>,
    pub settings: Vec<SettingOption>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            phone_books: vec![],
            settings: vec![],
        }
    }

    pub fn from_fake_data() -> Model {
        Model {
            phone_books: vec![PhoneBook {
                name: "phone book 1 test".to_string(),
                contacts: vec![
                    Contact {
                        name: "Alice".to_string(),
                        phone_number: "123456".to_string(),
                    },
                    Contact {
                        name: "Bob".to_string(),
                        phone_number: "567890".to_string(),
                    },
                ],
            }],
            settings: vec![SettingOption::Resolution("Resolution".to_string(), 0)],
        }
    }
}
