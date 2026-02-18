use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Secret {
    pub alias: String,
    pub encrypted_value: String, // In a real system, this would be encrypted at rest
}

pub struct SecretVault {
    #[allow(dead_code)]
    secrets: Arc<RwLock<HashMap<String, String>>>,
}

impl SecretVault {
    pub fn new() -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set_secret(&self, alias: &str, value: &str) {
        let mut w = self.secrets.write().unwrap();
        w.insert(alias.to_string(), value.to_string());
    }

    #[allow(dead_code)]
    pub fn get_secret(&self, alias: &str) -> Option<String> {
        let r = self.secrets.read().unwrap();
        r.get(alias).cloned()
    }

    #[allow(dead_code)]
    pub fn get_ephemeral_env(&self) -> HashMap<String, String> {
        let r = self.secrets.read().unwrap();
        r.clone()
    }

    #[allow(dead_code)]
    pub fn delete_secret(&self, alias: &str) {
        let mut w = self.secrets.write().unwrap();
        w.remove(alias);
    }
}
