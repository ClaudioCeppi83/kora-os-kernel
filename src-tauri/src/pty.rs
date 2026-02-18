use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct PtyManager {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

impl PtyManager {
    pub fn new(app: AppHandle) -> Self {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to open PTY");

        let shell = if cfg!(target_os = "windows") {
            "cmd.exe"
        } else {
            "bash"
        };
        
        let cmd = CommandBuilder::new(shell);
        let _child = pair.slave.spawn_command(cmd).expect("Failed to spawn shell");

        let mut reader = pair.master.try_clone_reader().expect("Failed to clone reader");
        let writer = pair.master.take_writer().expect("Failed to take writer");
        let writer = Arc::new(Mutex::new(writer));

        // Read loop
        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let data = String::from_utf8_lossy(&buf[..n]).to_string();
                        let _ = app.emit("pty-data", data);
                    }
                    Ok(_) => break, // EOF
                    Err(_) => break,
                }
            }
        });

        Self { writer }
    }

    pub fn write(&self, data: &str) {
        if let Ok(mut writer) = self.writer.lock() {
            let _ = writer.write_all(data.as_bytes());
            let _ = writer.flush();
        }
    }
}
