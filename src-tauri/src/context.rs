use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextState {
    pub active_workspace: Option<PathBuf>,
    pub focus_level: u8, // 0-100
    pub user_intent: String,
}

impl Default for ContextState {
    fn default() -> Self {
        Self {
            active_workspace: None,
            focus_level: 50,
            user_intent: "Idle".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ContextManager {
    state: Arc<Mutex<ContextState>>,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ContextState::default())),
        }
    }

    pub fn update_focus(&self, level: u8) {
        if let Ok(mut state) = self.state.lock() {
            state.focus_level = level.clamp(0, 100);
        }
    }

    pub fn set_workspace(&self, path: PathBuf) {
        if let Ok(mut state) = self.state.lock() {
            state.active_workspace = Some(path);
        }
    }

    pub fn set_intent(&self, intent: String) {
        if let Ok(mut state) = self.state.lock() {
            state.user_intent = intent;
        }
    }

    pub fn get_snapshot(&self) -> ContextState {
        self.state.lock().unwrap().clone()
    }
}
