use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Represents all possible actions that can be performed in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    NextTab,
    PreviousTab,
    NextItem,
    PreviousItem,
    NextPage,
    PreviousPage,
    ToggleDetailView,
    Back,
    Refresh,
    Noop,
}

/// Maps key events to actions
pub struct KeyMap {
    key_map: HashMap<KeyEvent, Action>,
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut key_map = HashMap::new();
        
        // Default key bindings
        key_map.insert(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE), Action::Quit);
        key_map.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), Action::Quit);
        key_map.insert(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE), Action::NextTab);
        key_map.insert(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT), Action::PreviousTab);
        key_map.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), Action::NextItem);
        key_map.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), Action::PreviousItem);
        key_map.insert(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE), Action::NextPage);
        key_map.insert(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE), Action::PreviousPage);
        key_map.insert(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), Action::ToggleDetailView);
        key_map.insert(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), Action::Back);
        key_map.insert(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE), Action::Refresh);
        
        Self { key_map }
    }
}

impl KeyMap {
    /// Creates a new KeyMap with default key bindings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Maps a key event to an action
    pub fn get_action(&self, key: KeyEvent) -> Action {
        *self.key_map.get(&key).unwrap_or(&Action::Noop)
    }
    
    /// Updates a key binding
    pub fn set_key_binding(&mut self, key: KeyEvent, action: Action) {
        self.key_map.insert(key, action);
    }
    
    /// Processes a key event and returns the corresponding action
    pub fn handle_key_event(&self, event: Event) -> Action {
        match event {
            Event::Key(key) => self.get_action(key),
            _ => Action::Noop,
        }
    }
    
    /// Returns a list of all key bindings
    pub fn get_key_bindings(&self) -> Vec<(KeyEvent, Action)> {
        self.key_map.iter().map(|(k, v)| (*k, *v)).collect()
    }
    
    /// Loads key bindings from a configuration
    pub fn load_from_config(&mut self, config: &HashMap<KeyEvent, Action>) {
        for (key, action) in config {
            self.key_map.insert(*key, *action);
        }
    }
}
