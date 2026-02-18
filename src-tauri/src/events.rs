use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::broadcast;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SystemEvent {
    FileChanged(PathBuf),
    SecurityAlert(String),
    PluginLoaded(String),
    ContextUpdate(String), // Placeholder for ContextState serialized or simplified
    SystemReady,
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100); // Buffer size 100
        Self { sender }
    }

    pub fn publish(&self, event: SystemEvent) {
        // We ignore the error if there are no subscribers
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.sender.subscribe()
    }
}
