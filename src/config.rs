use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DaliConfig {
    pub keybindings: Keybindings,
    pub theme: Theme,
    pub editor: EditorConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keybindings {
    pub quit: String,
    pub save: String,
    pub find: String,
    pub command: String,
    pub fuzzy: String,
}

use crossterm::event::{KeyCode, KeyModifiers};

impl Keybindings {
    pub fn parse_key(s: &str) -> (KeyCode, KeyModifiers) {
        let parts: Vec<&str> = s.split('-').collect();
        let mut modifiers = KeyModifiers::empty();
        let mut key_code = KeyCode::Null;

        for part in parts {
            match part.to_lowercase().as_str() {
                "ctrl" => modifiers.insert(KeyModifiers::CONTROL),
                "shift" => modifiers.insert(KeyModifiers::SHIFT),
                "alt" => modifiers.insert(KeyModifiers::ALT),
                s if s.len() == 1 => key_code = KeyCode::Char(s.chars().next().unwrap()),
                "enter" => key_code = KeyCode::Enter,
                "esc" => key_code = KeyCode::Esc,
                "backspace" => key_code = KeyCode::Backspace,
                "tab" => key_code = KeyCode::Tab,
                _ => {}
            }
        }
        (key_code, modifiers)
    }

    pub fn matches(&self, bind: &str, code: KeyCode, mods: KeyModifiers) -> bool {
        let (target_code, target_mods) = Self::parse_key(bind);
        code == target_code && mods.contains(target_mods)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub status_bar_bg: [u8; 3],
    pub status_bar_fg: [u8; 3],
    pub selection_bg: [u8; 3],
    pub gutter_fg: [u8; 3],
    pub keyword: [u8; 3],
    pub type_color: [u8; 3],
    pub string: [u8; 3],
    pub number: [u8; 3],
    pub comment: [u8; 3],
    pub normal: [u8; 3],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorConfig {
    pub tab_size: usize,
    pub line_numbers: bool,
}

impl Default for DaliConfig {
    fn default() -> Self {
        Self {
            keybindings: Keybindings {
                quit: "ctrl-q".to_string(),
                save: "ctrl-s".to_string(),
                find: "ctrl-f".to_string(),
                command: "ctrl-e".to_string(),
                fuzzy: "ctrl-p".to_string(),
            },
            theme: Theme {
                status_bar_bg: [0, 150, 150],
                status_bar_fg: [255, 255, 255],
                selection_bg: [60, 60, 100],
                gutter_fg: [100, 100, 100],
                keyword: [255, 120, 100],
                type_color: [100, 200, 255],
                string: [150, 255, 150],
                number: [255, 200, 100],
                comment: [120, 120, 120],
                normal: [255, 255, 255],
            },
            editor: EditorConfig {
                tab_size: 4,
                line_numbers: true,
            },
        }
    }
}

pub fn load_config() -> DaliConfig {
    let mut config_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_path.push("dali/dali.toml");

    if !config_path.exists() {
        let default_config = DaliConfig::default();
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(serialized) = toml::to_string(&default_config) {
            let _ = fs::write(&config_path, serialized);
        }
        return default_config;
    }

    if let Ok(content) = fs::read_to_string(config_path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        DaliConfig::default()
    }
}
