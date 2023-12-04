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
struct SettingItemState<T: Display + Clone> {
    name: String,
    current_selected_item_index: usize,
    items: Vec<T>,
}

impl<T: Display + Clone> SettingItemState<T> {
    pub fn with_one_item(name: String, option: T) -> SettingItemState<T> {
        SettingItemState {
            current_selected_item_index: 0,
            items: vec![option],
            name,
        }
    }

    pub fn new(name: String, options: Vec<T>) -> SettingItemState<T> {
        SettingItemState {
            current_selected_item_index: 0,
            items: options,
            name,
        }
    }

    pub fn value(&self) -> T {
        let index = self.current_selected_item_index;
        self.items.get(index).unwrap().clone()
    }

    // pub fn value_string(&self) -> String {
    //     format!(
    //         "{}",
    //         self.items.get(self.current_selected_item_index).unwrap()
    //     )
    // }
}

#[derive(Clone)]
pub struct SettingsState {
    resolution: SettingItemState<ResolutionOption>,
    refresh_rate: SettingItemState<RefreshRateOption>,
    v_sync: SettingItemState<VSyncOption>,
    language: SettingItemState<LanguageOption>,
    developer_mode: SettingItemState<DeveloperModeOption>,
}

impl SettingsState {
    pub fn default() -> SettingsState {
        SettingsState {
            resolution: SettingItemState::new(
                "Resolution".to_string(),
                vec![
                    ResolutionOption::_40_10,
                    ResolutionOption::_60_15,
                    ResolutionOption::_80_20,
                ],
            ),
            refresh_rate: SettingItemState::new(
                "Refresh Rate".to_string(),
                vec![
                    RefreshRateOption::_15,
                    RefreshRateOption::_30,
                    RefreshRateOption::_60,
                ],
            ),
            v_sync: SettingItemState::new(
                "V-Sync".to_string(),
                vec![VSyncOption::OFF, VSyncOption::ON],
            ),
            language: SettingItemState::new(
                "Language".to_string(),
                vec![LanguageOption::English, LanguageOption::Chinese],
            ),
            developer_mode: SettingItemState::new(
                "Developer Mode".to_string(),
                vec![DeveloperModeOption::OFF, DeveloperModeOption::ON],
            ),
        }
    }

    pub fn update_item_select(&mut self, name: String, new_select_item_index: usize) {
        match name.as_str() {
            "Resolution" => {
                if new_select_item_index < self.resolution.items.len() {
                    self.resolution.current_selected_item_index = new_select_item_index;
                }
            }
            "Refresh Rate" => {
                if new_select_item_index < self.refresh_rate.items.len() {
                    self.refresh_rate.current_selected_item_index = new_select_item_index;
                }
            }
            "V-Sync" => {
                if new_select_item_index < self.v_sync.items.len() {
                    self.v_sync.current_selected_item_index = new_select_item_index;
                }
            }
            "Language" => {
                if new_select_item_index < self.language.items.len() {
                    self.language.current_selected_item_index = new_select_item_index;
                }
            }
            "Developer Mode" => {
                if new_select_item_index < self.developer_mode.items.len() {
                    self.developer_mode.current_selected_item_index = new_select_item_index;
                }
            }
            _ => {}
        }
    }

    pub fn string_item_state_entries(&self) -> Vec<(String, usize, Vec<String>)> {
        vec![
            (
                self.resolution.name.clone(),
                self.resolution.current_selected_item_index,
                self.resolution
                    .items
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
            ),
            (
                self.refresh_rate.name.clone(),
                self.refresh_rate.current_selected_item_index,
                self.refresh_rate
                    .items
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
            ),
            (
                self.v_sync.name.clone(),
                self.v_sync.current_selected_item_index,
                self.v_sync
                    .items
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
            ),
            (
                self.language.name.clone(),
                self.language.current_selected_item_index,
                self.language
                    .items
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
            ),
            (
                self.developer_mode.name.clone(),
                self.developer_mode.current_selected_item_index,
                self.developer_mode
                    .items
                    .iter()
                    .map(|item| item.to_string())
                    .collect(),
            ),
        ]
    }

    pub fn string_entries(&self) -> Vec<(String, String)> {
        vec![
            (
                self.resolution.name.clone(),
                self.resolution.value().to_string(),
            ),
            (
                self.refresh_rate.name.clone(),
                self.refresh_rate.value().to_string(),
            ),
            (self.v_sync.name.clone(), self.v_sync.value().to_string()),
            (
                self.language.name.clone(),
                self.language.value().to_string(),
            ),
            (
                self.developer_mode.name.clone(),
                self.developer_mode.value().to_string(),
            ),
        ]
    }
}

pub struct SettingConfig {
    resolution: ResolutionOption,
    refresh_rate: RefreshRateOption,
    v_sync: VSyncOption,
    language: LanguageOption,
    developer_mode: DeveloperModeOption,
}

impl SettingConfig {
    pub fn from_state(state: SettingsState) -> SettingConfig {
        SettingConfig {
            resolution: state.resolution.value(),
            refresh_rate: state.refresh_rate.value(),
            v_sync: state.v_sync.value(),
            language: state.language.value(),
            developer_mode: state.developer_mode.value(),
        }
    }
}

pub struct ModelCache {
    pub current_select_setting_index: usize,
}

pub struct Model {
    pub phone_books: Vec<PhoneBook>,
    pub settings: SettingsState,
    pub cache: ModelCache,
}

impl Model {
    pub fn new() -> Model {
        Model {
            phone_books: vec![],
            settings: SettingsState::default(),
            cache: ModelCache {
                current_select_setting_index: 0,
            },
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
            settings: SettingsState::default(),
            cache: ModelCache {
                current_select_setting_index: 0,
            },
        }
    }
}
