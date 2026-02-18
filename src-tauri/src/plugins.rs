#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub permissions: Vec<String>,
    pub entry_point: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct LoadedPlugin {
    pub manifest: PluginManifest,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum PluginError {
    IoError(String),
    InvalidManifest(String),
    PermissionDenied(String),
}

pub struct PluginLoader {
    plugins_dir: PathBuf,
}

impl PluginLoader {
    pub fn new(base_path: &Path) -> Self {
        Self {
            plugins_dir: base_path.join("plugins"),
        }
    }

    pub fn load_all(&self) -> Vec<Result<LoadedPlugin, PluginError>> {
        let mut results = Vec::new();
        let path = Path::new(&self.plugins_dir);

        if !path.exists() {
            // Create if not exists (or just return empty)
            let _ = fs::create_dir_all(path);
            return results;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    results.push(self.load_plugin(&entry_path));
                }
            }
        }

        results
    }

    fn load_plugin(&self, plugin_path: &Path) -> Result<LoadedPlugin, PluginError> {
        let manifest_path = plugin_path.join("manifest.json");

        if !manifest_path.exists() {
            return Err(PluginError::InvalidManifest("Missing manifest.json".into()));
        }

        let content =
            fs::read_to_string(&manifest_path).map_err(|e| PluginError::IoError(e.to_string()))?;

        let manifest: PluginManifest = serde_json::from_str(&content)
            .map_err(|e| PluginError::InvalidManifest(format!("Parse error: {}", e)))?;

        // Validate permissions (Security Gate Ref)
        // For now, we just ensure they don't ask for "ROOT" or "RING_0" directly without signature?
        // Basic check:
        // Assume permissions are vectors of strings.
        for perm in &manifest.permissions {
            if perm == "RING_0" {
                return Err(PluginError::PermissionDenied(
                    "RING_0 Permission forbidden for plugins".into(),
                ));
            }
        }

        Ok(LoadedPlugin {
            manifest,
            path: plugin_path.to_path_buf(),
        })
    }
}
