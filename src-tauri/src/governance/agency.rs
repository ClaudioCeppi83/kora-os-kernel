use std::sync::{Arc, RwLock}; // RwLock is faster for reads
use tauri::{AppHandle, Manager, Emitter};
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::audit;

/// Represents an administrative agency within KORA OS.
#[derive(Clone, Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Agency {
    /// The unique agency identifier (e.g., 'SYSTEM', 'RED_TEAM').
    pub id: String,
    /// Human-readable name of the agency.
    pub name: String,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
}

/// Manages the active agency context and handles secure switching between agencies.
#[derive(Clone)]
pub struct AgencyManager {
    active_agency_id: Arc<RwLock<String>>,
    app_handle: AppHandle,
}

impl AgencyManager {
    pub fn new(app_handle: AppHandle) -> Self {
        AgencyManager {
            active_agency_id: Arc::new(RwLock::new("SYSTEM".to_string())),
            app_handle,
        }
    }

    pub fn get_active_agency_id(&self) -> String {
        self.active_agency_id.read().unwrap().clone()
    }
    
    // Commands implementation logic is separated to keep this struct clean if needed, 
    // but for now we put helpers here.

    /// Switches the active system context to a different agency.
    ///
    /// This is a high-security operation that locks the communication bridge,
    /// notifies the UI, and updates the global agency state.
    pub async fn switch_agency(&self, app_state: &tauri::State<'_, AppState>, new_agency_id: String) -> Result<String, String> {
        // 1. Security Protocol: Lock Bridge immediately
        app_state.bridge_locked.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // 2. UI Notification
        let _ = self.app_handle.emit("context-switching", &new_agency_id);

        // 3. Flush (simulated delay for now, real flush happens via DB transactions usually immediate)
        // In a real scenario, we might wait for pending async tasks.
        
        // 4. Update State
        {
            let mut w = self.active_agency_id.write().map_err(|e| e.to_string())?;
            *w = new_agency_id.clone();
        }

        // 5. Audit
        let _ = audit::log_event(&app_state.db, "CONTEXT_SWITCH", "RING_1", &format!("Switched to {}", new_agency_id), &new_agency_id).await;
        
        // 6. Clear Memory (Simulated memory wipe of OpenClaw context if we had a direct handle, 
        // effectively handled by next command sent or explicitly sending RESET to engine)
        // For now, we assume the engine is stateless per request or we send a reset command next.

        Ok(format!("Switched to {}", new_agency_id))
    }
}

// Commands to be registered in lib.rs

#[tauri::command]
pub async fn kora_agency_create(app: tauri::AppHandle, state: tauri::State<'_, AppState>, name: String) -> Result<String, String> {
    // Basic ID generation
    let id = name.to_uppercase().replace(" ", "_");
    
    // DB Insert
    // We need to use sqlx query here. 
    // Ideally this logic belongs in db.rs or here using &state.db
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    sqlx::query("INSERT INTO agencies (id, name, created_at, is_active) VALUES (?, ?, ?, 1)")
        .bind(&id)
        .bind(&name)
        .bind(&timestamp)
        .execute(&state.db)
        .await
        .map_err(|e| format!("DB Error: {}", e))?;

    // FS Scaffolding
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let know_path = app_data_dir.join("knowledge").join(&id);
    let work_path = app_data_dir.join("workspace").join(&id);
    
    std::fs::create_dir_all(know_path).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(work_path).map_err(|e| e.to_string())?;

    // Audit
    let _ = crate::audit::log_event(&state.db, "AGENCY_CREATE", "RING_0", &id, &id).await;

    Ok(format!("Agency {} created", id))
}

#[tauri::command]
pub async fn kora_agency_list(state: tauri::State<'_, AppState>) -> Result<Vec<Agency>, String> {
    let agencies = sqlx::query_as::<_, Agency>("SELECT id, name, created_at FROM agencies")
        .fetch_all(&state.db)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;
    Ok(agencies)
}

#[tauri::command]
pub async fn cmd_shutdown(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
    // 1. Audit Shutdown (Immutable Log)
    let agency_id = state.governance.get_active_agency_id();
    let _ = crate::audit::log_event(&state.db, "SYSTEM_HALT", "RING_0", "Secure Shutdown Initiated", &agency_id).await;
    
    // 2. Final Snapshot (Persistence)
    let _ = crate::db::save_session_snapshot(&state.db, &agency_id, "SYSTEM_HALT", "APPROVED", "SHUTDOWN").await;

    // 3. Graceful Exit Protocol
    let pool = state.db.clone();
    tokio::spawn(async move {
        println!("[KORA] Closing Database Pool...");
        let _ = pool.close().await;
        println!("[KORA] Halting Kernel...");
        // Delay slightly for output flush, then use Tauri's native exit
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        app.exit(0);
    });

    Ok(())
}

#[tauri::command]
pub async fn kora_agency_switch(state: tauri::State<'_, AppState>, id: String) -> Result<String, String> {
    // Use the manager method
    state.governance.switch_agency(&state, id).await
}
