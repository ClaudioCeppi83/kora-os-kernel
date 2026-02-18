use std::path::{Path, PathBuf};
use tauri::State;
use crate::AppState;

#[derive(Debug)]
#[allow(dead_code)]
pub enum JailError {
    AccessDenied(String),
    InvalidPath(String),
    JailBreakAttempt(String),
}

pub struct KoraJail {
    whitelist: Vec<PathBuf>,
}

impl KoraJail {
    pub fn new(agency_id: &str) -> Self {
        // Dynamic whitelist based on Active Agency (Ring 1 Isolation)
        let whitelist = vec![
            PathBuf::from(format!("/knowledge/{}", agency_id)),
            PathBuf::from(format!("/workspace/{}", agency_id)),
            PathBuf::from("/data"), // Shared system data
            PathBuf::from("/logs"), // Shared audit logs
        ];

        Self { whitelist }
    }

    /// Validates if a path is within the whitelist and does not contain traversals
    pub fn validate_path(&self, path_str: &str) -> Result<PathBuf, JailError> {
        let path = Path::new(path_str);

        // Check for traversal attempts
        if path_str.contains("..") {
            return Err(JailError::JailBreakAttempt(format!("Path traversal detected: {}", path_str)));
        }

        // Check if path starts with any allowed root
        // We use string matching to enforce the prefix strictly
        let allowed = self.whitelist.iter().any(|root| {
            path_str.starts_with(root.to_str().unwrap_or(""))
        });

        if !allowed {
             return Err(JailError::AccessDenied(format!("Path not in agency whitelist: {}", path_str)));
        }

        Ok(path.to_path_buf())
    }
}

/// Helper to actively enforcement jail check and trigger Hard Reset on failure
pub async fn enforce(state: &State<'_, AppState>, path: &str, operation: &str) -> Result<PathBuf, String> {
    // Get Active Agency from Ring 1 Governance
    let agency_id = state.governance.get_active_agency_id();
    let jail = KoraJail::new(&agency_id);
    
    match jail.validate_path(path) {
        Ok(p) => Ok(p),
        Err(e) => {
            let error_msg = format!("{:?}", e);
            
            // LOG SECURITY VIOLATION (Immutable Audit)
            let _ = crate::audit::log_event(
                &state.db,
                "JAIL_VIOLATION",
                "RING_1",
                &format!("Unauthorized path: {}, Op: {}, Error: {}", path, operation, error_msg),
                &agency_id
            ).await;
            
            // TRIGGER HARD RESET (Simulated by setting bridge lock and emitting fatal error)
            // In a real OS, this would kill the process. Here we lock the UI.
            state.bridge_locked.store(true, std::sync::atomic::Ordering::SeqCst);
            
            // TODO: Emit HARD_RESET event to frontend
            
            Err(format!("SECURITY HARD RESET TRIGGERED: {}", error_msg))
        }
    }
}
