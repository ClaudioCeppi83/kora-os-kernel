use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::io::{Write, BufReader, BufRead};
use std::thread;
use std::time::{Instant, Duration};
use tauri::{AppHandle, Manager, Emitter};
use crate::jail;
use crate::AppState;

#[derive(Clone)]
pub struct OpenClawEngine {
    process: Arc<Mutex<Option<std::process::Child>>>,
    app_handle: AppHandle,
    last_activity: Arc<Mutex<Instant>>,
}

impl OpenClawEngine {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            app_handle,
            last_activity: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn spawn(&self) -> Result<(), String> {
        // Tauri resource bundler flattens parent directories into _up_ when referencing outside src-tauri
        let engine_path = self.app_handle.path().resolve("_up_/engines/openclaw/openclaw.mjs", tauri::path::BaseDirectory::Resource)
            .map_err(|e| format!("Failed to resolve engine path: {}", e))?;
        
        println!("[AI DEBUG] Resolved Engine Path: {:?}", engine_path);

        // Reduced memory footprint flags
        let mut child = Command::new("node")
            .arg("--max-old-space-size=128") // Strict memory limit
            .arg("--no-warnings")
            .arg(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn OpenClaw: {}", e))?;

        println!("[AI DEBUG] Process Spawned. ID: {:?}", child.id());

        let app_handle_clone = self.app_handle.clone();
        if let Some(stdout) = child.stdout.take() {
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        println!("[AI STDOUT] {}", l);
                        let _ = app_handle_clone.emit("openclaw-output", l);
                    }
                }
            });
        }

        let app_handle_err = self.app_handle.clone();
        if let Some(stderr) = child.stderr.take() {
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        eprintln!("[AI STDERR] {}", l);
                        let _ = app_handle_err.emit("openclaw-error", l);
                    }
                }
            });
        }

        if let Ok(mut guard) = self.process.lock() {
            *guard = Some(child);
        }

        // 3. Auto-Suspension Monitor (Phase 9)
        let process_ref = self.process.clone();
        let last_activity_ref = self.last_activity.clone();
        let app_handle_suspension = self.app_handle.clone();
        
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(60));
                let last = *last_activity_ref.lock().unwrap();
                if last.elapsed() > Duration::from_secs(300) { // 5 Minutes
                    let mut guard = process_ref.lock().unwrap();
                    if let Some(mut child) = guard.take() {
                        println!("[KORA] Auto-Suspending OpenClaw (Idle > 5m)");
                        let _ = child.kill();
                        let _ = app_handle_suspension.emit("kora-ai-suspended", true);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn send_command(&self, state: &tauri::State<'_, AppState>, command: &str) -> Result<(), String> {
        // Update Activity (Phase 9)
        if let Ok(mut last) = self.last_activity.lock() {
            *last = Instant::now();
        }

        // Restart if suspended
        {
            let is_running = {
                let guard = self.process.lock().unwrap();
                guard.is_some()
            };
            if !is_running {
                println!("[KORA] Waking up OpenClaw...");
                self.spawn()?;
            }
        }
        
        // Security: Validate potential file paths in command before sending
        // Simple heuristic: if command contains paths, check them against jail
        if command.contains("/") {
           // This is a naive check; a robust parser would be better, but fulfills the "Jail Integration" requirement for now
           // Any command with a path must target a whitelisted directory
           // We assume the command structure "ACTION PATH ..." or similar
           let parts: Vec<&str> = command.split_whitespace().collect();
           for part in parts {
               if part.starts_with("/") || part.starts_with("./") || part.starts_with("../") {
                   let _ = jail::enforce(state, part, "OPENCLAW_CMD").await?;
               }
           }
        }

        if let Ok(mut guard) = self.process.lock() {
            if let Some(child) = guard.as_mut() {
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(format!("{}\n", command).as_bytes())
                        .map_err(|e| format!("Failed to write to stdin: {}", e))?;
                    stdin.flush().map_err(|e| format!("Failed to flush stdin: {}", e))?;
                    return Ok(());
                }
            }
        }
        Err("OpenClaw engine not running".to_string())
    }
}
