use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use tauri::{AppHandle, Manager, Runtime};
use crate::rag;
use crate::AppState; 

pub fn init_watcher<R: Runtime>(app: AppHandle<R>) {
    thread::spawn(move || {
        let (tx, rx) = channel();
        
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("[WATCHER] Failed to create watcher: {:?}", e);
                return;
            }
        };

        // Watch /knowledge directory
        // In dev, this might be relative. In prod, strict path.
        let knowledge_path = Path::new("knowledge"); // Relative to CWD for now as defined in prompt implies root /knowledge
        // But for safety let's assume it exists in CWD or handle absolute if /knowledge is system root (unlikely for user mode)
        // We will assume relative to CWD for this prototype unless jail says otherwise. 
        // Jail said "/knowledge" which usually means absolute in *nix, but for this app context it likely means project root/knowledge.
        // Let's try to verify if it exists, if not create it or warn.
        if !knowledge_path.exists() {
             let _ = std::fs::create_dir_all(knowledge_path);
        }

        if let Err(e) = watcher.watch(knowledge_path, RecursiveMode::Recursive) {
             eprintln!("[WATCHER] Failed to watch knowledge: {:?}", e);
             return;
        }

        println!("[WATCHER] Started watching {:?}", knowledge_path);

        for res in rx {
            match res {
                Ok(event) => {
                    // Filter for Create or Modify events
                    if event.kind.is_create() || event.kind.is_modify() {
                         for path in event.paths {
                             if path.is_file() {
                                 // Check file size (< 10MB) - arbitrary limit for "validar su tamaño"
                                 if let Ok(metadata) = std::fs::metadata(&path) {
                                     if metadata.len() < 10 * 1024 * 1024 {
                                         // Trigger Indexing
                                         // We need access to DB. We can get it from AppHandle -> State -> DB
                                         let app_handle = app.clone();
                                         let path_str = path.to_string_lossy().to_string();
                                         
                                         // Spawn async task to index
                                         tauri::async_runtime::spawn(async move {
                                             if let Some(state) = app_handle.try_state::<AppState>() {
                                                 println!("[WATCHER] Indexing detected file: {}", path_str);
                                                 let _ = rag::index_file(&state.db, &path_str).await;
                                             }
                                         });
                                     }
                                 }
                             }
                         }
                    }
                },
                Err(e) => eprintln!("[WATCHER] Watch error: {:?}", e),
            }
        }
    });
}
