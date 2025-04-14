use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

use super::keymap::{Action, KeyMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyConfig {
    pub quit: Vec<KeyDescription>,
    pub next_tab: Vec<KeyDescription>,
    pub previous_tab: Vec<KeyDescription>,
    pub next_item: Vec<KeyDescription>,
    pub previous_item: Vec<KeyDescription>,
    pub next_page: Vec<KeyDescription>,
    pub previous_page: Vec<KeyDescription>,
    pub toggle_detail: Vec<KeyDescription>,
    pub back: Vec<KeyDescription>,
    pub refresh: Vec<KeyDescription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDescription {
    pub key: String,
    pub modifiers: Vec<String>,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            quit: vec![
                KeyDescription {
                    key: "q".to_string(),
                    modifiers: vec![],
                },
                KeyDescription {
                    key: "c".to_string(),
                    modifiers: vec!["CONTROL".to_string()],
                },
            ],
            next_tab: vec![KeyDescription {
                key: "Tab".to_string(),
                modifiers: vec![],
            }],
            previous_tab: vec![KeyDescription {
                key: "BackTab".to_string(),
                modifiers: vec!["SHIFT".to_string()],
            }],
            next_item: vec![KeyDescription {
                key: "Down".to_string(),
                modifiers: vec![],
            }],
            previous_item: vec![KeyDescription {
                key: "Up".to_string(),
                modifiers: vec![],
            }],
            next_page: vec![KeyDescription {
                key: "PageDown".to_string(),
                modifiers: vec![],
            }],
            previous_page: vec![KeyDescription {
                key: "PageUp".to_string(),
                modifiers: vec![],
            }],
            toggle_detail: vec![KeyDescription {
                key: "Enter".to_string(),
                modifiers: vec![],
            }],
            back: vec![KeyDescription {
                key: "Esc".to_string(),
                modifiers: vec![],
            }],
            refresh: vec![KeyDescription {
                key: "r".to_string(),
                modifiers: vec![],
            }],
        }
    }
}

impl KeyConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let config = toml::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        Ok(config)
    }
    
    pub fn to_key_map(&self) -> KeyMap {
        let mut key_map = KeyMap::new();
        
        // Process each key configuration
        self.process_key_descriptions(&mut key_map, &self.quit, Action::Quit);
        self.process_key_descriptions(&mut key_map, &self.next_tab, Action::NextTab);
        self.process_key_descriptions(&mut key_map, &self.previous_tab, Action::PreviousTab);
        self.process_key_descriptions(&mut key_map, &self.next_item, Action::NextItem);
        self.process_key_descriptions(&mut key_map, &self.previous_item, Action::PreviousItem);
        self.process_key_descriptions(&mut key_map, &self.next_page, Action::NextPage);
        self.process_key_descriptions(&mut key_map, &self.previous_page, Action::PreviousPage);
        self.process_key_descriptions(&mut key_map, &self.toggle_detail, Action::ToggleDetailView);
        self.process_key_descriptions(&mut key_map, &self.back, Action::Back);
        self.process_key_descriptions(&mut key_map, &self.refresh, Action::Refresh);
        
        key_map
    }
    
    fn process_key_descriptions(&self, key_map: &mut KeyMap, descriptions: &[KeyDescription], action: Action) {
        for desc in descriptions {
            if let Some(key_event) = self.parse_key_description(desc) {
                key_map.set_key_binding(key_event, action);
            }
        }
    }
    
    fn parse_key_description(&self, desc: &KeyDescription) -> Option<KeyEvent> {
        let key_code = match desc.key.as_str() {
            // Special keys
            "Enter" => KeyCode::Enter,
            "Tab" => KeyCode::Tab,
            "BackTab" => KeyCode::BackTab,
            "Backspace" => KeyCode::Backspace,
            "Esc" => KeyCode::Esc,
            "Left" => KeyCode::Left,
            "Right" => KeyCode::Right,
            "Up" => KeyCode::Up,
            "Down" => KeyCode::Down,
            "Home" => KeyCode::Home,
            "End" => KeyCode::End,
            "PageUp" => KeyCode::PageUp,
            "PageDown" => KeyCode::PageDown,
            "Delete" => KeyCode::Delete,
            "Insert" => KeyCode::Insert,
            "F1" => KeyCode::F(1),
            "F2" => KeyCode::F(2),
            "F3" => KeyCode::F(3),
            "F4" => KeyCode::F(4),
            "F5" => KeyCode::F(5),
            "F6" => KeyCode::F(6),
            "F7" => KeyCode::F(7),
            "F8" => KeyCode::F(8),
            "F9" => KeyCode::F(9),
            "F10" => KeyCode::F(10),
            "F11" => KeyCode::F(11),
            "F12" => KeyCode::F(12),
            // Single character
            s if s.len() == 1 => {
                let c = s.chars().next().unwrap();
                KeyCode::Char(c)
            }
            _ => return None,
        };
        
        let mut modifiers = KeyModifiers::NONE;
        for modifier in &desc.modifiers {
            match modifier.as_str() {
                "SHIFT" => modifiers |= KeyModifiers::SHIFT,
                "CONTROL" => modifiers |= KeyModifiers::CONTROL,
                "ALT" => modifiers |= KeyModifiers::ALT,
                _ => {}
            }
        }
        
        Some(KeyEvent::new(key_code, modifiers))
    }
}
