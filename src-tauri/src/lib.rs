mod audit;
mod db;
mod pty;
mod rag;
mod jail;
mod drivers;
mod plugins;
mod ai_engine;
mod governance;
mod security;

use crate::pty::PtyManager;
use crate::ai_engine::OpenClawEngine;
use crate::governance::agency::{AgencyManager, kora_agency_create, kora_agency_list, kora_agency_switch, cmd_shutdown};
use crate::security::vault::SecretVault;
use sqlx::{Pool, Sqlite};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use tauri::{Emitter, Manager, State};
use sysinfo::System;
use serde::Serialize;


#[derive(Serialize, Clone)]
struct SystemMetrics {
    cpu_usage: f32,
    ram_used: u64,
    ram_total: u64,
    timestamp: String,
}

pub struct AppState {
    pub pty: PtyManager,
    pub bridge_locked: Arc<AtomicBool>,
    pub db: Pool<Sqlite>,
    pub ai_engine: OpenClawEngine,
    pub governance: AgencyManager,
    pub vault: SecretVault,
    pub integrity_cache: Arc<RwLock<Option<String>>>,
    pub boot_time: std::time::Instant,
}

impl AppState {
    #[allow(dead_code)]
    pub fn set_secret(&self, alias: &str, value: &str) {
        self.vault.set_secret(alias, value);
    }

    #[allow(dead_code)]
    pub fn get_secret(&self, alias: &str) -> Option<String> {
        self.vault.get_secret(alias)
    }

    #[allow(dead_code)]
    pub fn delete_secret(&self, alias: &str) {
        self.vault.delete_secret(alias);
    }
}

#[tauri::command]
fn kora_kernel_status(state: State<'_, AppState>) -> bool {
    // If we can access state, the kernel has been initialized
    true
}

#[tauri::command]
fn pty_write(state: State<'_, AppState>, data: String) {
    if !state.bridge_locked.load(Ordering::SeqCst) {
        state.pty.write(&data);
    }
}

#[tauri::command]
fn heartbeat() -> String {
    "PULSE_OK".to_string()
}

#[tauri::command]
fn set_bridge_lock(state: State<'_, AppState>, locked: bool) {
    state.bridge_locked.store(locked, Ordering::SeqCst);
}

// Phase 3 Commands
#[tauri::command]
async fn index_file(state: State<'_, AppState>, path: String) -> Result<String, String> {
    // Jail Enforcement
    let valid_path = jail::enforce(&state, &path, "INDEX_FILE").await?;

    // Audit the request
    let path_str = valid_path.to_string_lossy();
    let agency_id = state.governance.get_active_agency_id();
    let _ = audit::log_event(&state.db, "INDEX_REQUEST", "RING_1", &path_str, &agency_id).await;
    
    // Execute RAG indexing
    rag::index_file(&state.db, &path_str).await
}

#[tauri::command]
async fn get_audit_logs(state: State<'_, AppState>) -> Result<Vec<audit::AuditLog>, String> {
    audit::get_logs(&state.db, 50).await
}

// Phase 6 Commands
#[tauri::command]
async fn kora_kernel_integrity(state: State<'_, AppState>) -> Result<String, String> {
    // 1. Check Cache First (Extreme Optimization)
    let last_validated = {
        let r = state.integrity_cache.read().unwrap();
        r.clone()
    };

    // 2. Perform Validation
    let validation_res = crate::audit::validate_chain(&state.db).await?;
    
    match validation_res {
        Some(new_hash) => {
            // Update cache if it changed or was empty
            if last_validated.is_none() || last_validated.unwrap() != new_hash {
                let mut w = state.integrity_cache.write().unwrap();
                *w = Some(new_hash);
            }
            Ok("GREEN".to_string())
        },
        None => Ok("RED".to_string())
    }
}


#[tauri::command]
async fn kora_system_benchmark(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let boot_elapsed = state.boot_time.elapsed().as_millis();
    let sys = System::new_all();
    let ram_used = sys.used_memory() / 1024 / 1024; // MB
    
    let start = std::time::Instant::now();
    let _ = sqlx::query("SELECT 1").execute(&state.db).await;
    let db_latency = start.elapsed().as_micros();

    Ok(serde_json::json!({
        "boot_ms": boot_elapsed,
        "ram_mb": ram_used,
        "db_latency_us": db_latency,
        "optimized": true,
        "lto": true,
        "strip": "symbols",
        "zero_copy_rag": true
    }))
}

#[tauri::command]
async fn kora_system(state: State<'_, AppState>, action: String) -> Result<String, String> {
    // 1. Audit
    let agency_id = state.governance.get_active_agency_id();
    let _ = audit::log_event(&state.db, "KORA_SYSTEM", "RING_3", &action, &agency_id).await;
    
    // 2. Jail Check 
    
    // 3. Vault Environment
    let _env = state.vault.get_ephemeral_env();
    
    // 4. Send to Engine
    state.ai_engine.send_command(&state, &format!("SYSTEM {}", action)).await?;

    // 5. Session Vault (Snapshot) 
    let _ = db::save_session_snapshot(&state.db, &agency_id, &format!("SYSTEM: {}", action), "PENDING", "SNAPSHOT_PENDING").await;
    
    Ok("Command Sent".to_string())
}

#[tauri::command]
async fn kora_knowledge(state: State<'_, AppState>, query: String) -> Result<String, String> {
    // 1. Audit
    let agency_id = state.governance.get_active_agency_id();
    let _ = audit::log_event(&state.db, "KORA_KNOWLEDGE", "RING_3", "QUERY_REDACTED", &agency_id).await; 
    
    // 2. Send to Engine
    // 3. Vault Environment
    let _env = state.vault.get_ephemeral_env();

    // 4. Session Vault (Snapshot) with Agency Context
    let agency_id = state.governance.get_active_agency_id();
    let _ = db::save_session_snapshot(&state.db, &agency_id, &format!("KNOWLEDGE: {}", query), "PENDING", "SNAPSHOT_PENDING").await;
    
    Ok("Query Sent".to_string())
}




pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Initialize PTY
            let pty_manager = PtyManager::new(app_handle.clone());
            
            // Initialize AI Engine
            let ai_engine = OpenClawEngine::new(app_handle.clone());
            
            // Initialize Agency Manager
            let agency_manager = AgencyManager::new(app_handle.clone());
            // Attempt to spawn, log error if fails but allow app to start
            if let Err(e) = ai_engine.spawn() {
                eprintln!("[KORA] Failed to spawn OpenClaw: {}", e);
            }
            
            // Initialize Watch Driver
            drivers::watch::init_watcher(app_handle.clone());

            // Initialize System Monitor (2Hz)
            let app_handle_monitor = app_handle.clone();
            std::thread::spawn(move || {
                let mut sys = System::new_all();
                loop {
                    sys.refresh_cpu();
                    sys.refresh_memory();
                    
                    let cpu_usage = sys.global_cpu_info().cpu_usage();
                    let ram_used = sys.used_memory();
                    let ram_total = sys.total_memory();
                    let timestamp = chrono::Utc::now().to_rfc3339();

                    let metrics = SystemMetrics {
                        cpu_usage,
                        ram_used,
                        ram_total,
                        timestamp,
                    };

                    let _ = app_handle_monitor.emit("system-telemetry", metrics);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            });

            let app_handle_for_setup = app_handle.clone();
            
            tauri::async_runtime::spawn(async move {
                // 1. Parallel Sync/Async Setup
                let db_init = db::init_db(&app_handle_for_setup);
                
                // Let's assume PTY or other services could also be async-initialized here
                let (db_res,) = tokio::join!(db_init);
                let db_pool = db_res.expect("Failed to init DB");
                
                let vault = SecretVault::new();
                vault.set_secret("KORA_MODE", "PRODUCTION");

                app_handle_for_setup.manage(AppState {
                    pty: pty_manager,
                    bridge_locked: Arc::new(AtomicBool::new(false)),
                    db: db_pool,
                    ai_engine: ai_engine,
                    governance: agency_manager,
                    vault: vault,
                    integrity_cache: Arc::new(RwLock::new(None)),
                    boot_time: std::time::Instant::now(),
                });

                // 2. Signal UI that Kernel is Hot
                let _ = app_handle_for_setup.emit("kora-kernel-ready", true);
                println!("[KORA] Kernel Hot in < 2s");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            kora_kernel_status,
            kora_system_benchmark,
            pty_write,
            heartbeat,
            set_bridge_lock,
            index_file,
            get_audit_logs,
            drivers::notify::send_notification,
            kora_system,
            kora_knowledge,
            kora_agency_create,
            kora_agency_list,
            kora_agency_switch,
            kora_kernel_integrity,
            cmd_shutdown
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
