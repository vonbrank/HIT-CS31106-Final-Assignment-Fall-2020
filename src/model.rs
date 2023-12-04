use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Contact {
    pub name: String,
    phone_number: String,
}

#[derive(Clone)]
pub struct PhoneBook {
    pub name: String,
    pub contacts: Vec<Contact>,
}
#[derive(Clone)]
pub enum ResolutionOption {
    _40_10,
    _60_15,
    _80_20,
}

impl Display for ResolutionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResolutionOption::_40_10 => "40 x 10".to_string(),
                ResolutionOption::_60_15 => "60 x 15".to_string(),
                ResolutionOption::_80_20 => "80 x 20".to_string(),
            }
        )
    }
}
#[derive(Clone)]
pub enum RefreshRateOption {
    _15,
    _30,
    _60,
}

impl Display for RefreshRateOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RefreshRateOption::_15 => "15 Hz".to_string(),
                RefreshRateOption::_30 => "30 Hz".to_string(),
                RefreshRateOption::_60 => "60 Hz".to_string(),
            }
        )
    }
}
#[derive(Clone)]
pub enum VSyncOption {
    ON,
    OFF,
}

impl Display for VSyncOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VSyncOption::ON => "ON".to_string(),
                VSyncOption::OFF => "OFF".to_string(),
            }
        )
    }
}
#[derive(Clone)]
pub enum LanguageOption {
    Chinese,
    English,
}

impl Display for LanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LanguageOption::Chinese => "中文".to_string(),
                LanguageOption::English => "English".to_string(),
            }
        )
    }
}
#[derive(Clone)]
pub enum DeveloperModeOption {
    ON,
    OFF,
}

impl Display for DeveloperModeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DeveloperModeOption::ON => "ON".to_string(),
                DeveloperModeOption::OFF => "OFF".to_string(),
            }
        )
    }
}

// pub enum SettingOption {
//     Resolution(String, i32),
//     RefreshRate(String, i32),
//     VSync(String, i32),
//     Language(String, i32),
//     DeveloperMode(String, i32),
// }
#[derive(Clone)]
struct SettingOption<T: Display> {
    name: String,
    current_selected_item_index: usize,
    items: Vec<T>,
}

impl<T: Display> SettingOption<T> {
    pub fn with_one_item(name: String, option: T) -> SettingOption<T> {
        SettingOption {
            current_selected_item_index: 0,
            items: vec![option],
            name,
        }
    }

    pub fn get_value_string(&self) -> String {
        format!(
            "{}",
            self.items.get(self.current_selected_item_index).unwrap()
        )
    }
}

#[derive(Clone)]
pub struct Settings {
    resolution: SettingOption<ResolutionOption>,
    refresh_rate: SettingOption<RefreshRateOption>,
    v_sync: SettingOption<VSyncOption>,
    language: SettingOption<LanguageOption>,
    developer_mode: SettingOption<DeveloperModeOption>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            resolution: SettingOption::with_one_item(
                "Resolution".to_string(),
                ResolutionOption::_40_10,
            ),
            refresh_rate: SettingOption::with_one_item(
                "Refresh Rate".to_string(),
                RefreshRateOption::_15,
            ),
            v_sync: SettingOption::with_one_item("V-Sync".to_string(), VSyncOption::OFF),
            language: SettingOption::with_one_item("Language".to_string(), LanguageOption::English),
            developer_mode: SettingOption::with_one_item(
                "Developer Mode".to_string(),
                DeveloperModeOption::OFF,
            ),
        }
    }

    pub fn setting_list(&self) -> Vec<(String, String)> {
        vec![
            (
                self.resolution.name.clone(),
                self.resolution.get_value_string(),
            ),
            (
                self.refresh_rate.name.clone(),
                self.refresh_rate.get_value_string(),
            ),
            (self.v_sync.name.clone(), self.v_sync.get_value_string()),
            (self.language.name.clone(), self.language.get_value_string()),
            (
                self.developer_mode.name.clone(),
                self.developer_mode.get_value_string(),
            ),
        ]
    }
}

pub struct Model {
    pub phone_books: Vec<PhoneBook>,
    pub settings: Settings,
}

impl Model {
    pub fn new() -> Model {
        Model {
            phone_books: vec![],
            settings: Settings::new(),
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
            settings: Settings::new(),
        }
    }
}
