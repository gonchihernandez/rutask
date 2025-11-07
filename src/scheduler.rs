use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::storage::TaskStorage;

pub struct Scheduler {
    storage: Arc<Mutex<TaskStorage>>,
    running: Arc<Mutex<bool>>,
}

impl Scheduler {
    pub fn new(storage: Arc<Mutex<TaskStorage>>) -> Self {
        Self {
            storage,
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            println!("⚠️ El scheduler ya está corriendo");
            return;
        }
        *running = true;
        drop(running);

        let storage = Arc::clone(&self.storage);
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            println!("🚀 Scheduler iniciado");
            
            loop {
                {
                    let is_running = running.lock().unwrap();
                    if !*is_running {
                        break;
                    }
                }

                // Revisar tareas pendientes cada 30 segundos
                Self::check_due_tasks(&storage);
                thread::sleep(Duration::from_secs(30));
            }
            
            println!("🛑 Scheduler detenido");
        });
    }

    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
        println!("⏸️ Deteniendo scheduler...");
    }

    fn check_due_tasks(storage: &Arc<Mutex<TaskStorage>>) {
        let mut storage = storage.lock().unwrap();
        let tasks = storage.get_all_tasks_mut();
        
        for task in tasks.iter_mut() {
            if task.is_completed() {
                continue;
            }

            if task.is_due() {
                Self::send_notification(task);
                task.mark_reminder_sent();
            }
        }
    }

    fn send_notification(task: &crate::task::Task) {
        println!("\n🔔 ═══════════════════════════════════");
        println!("   ⏰ RECORDATORIO");
        println!("   📝 Tarea: {}", task.title);
        
        if let Some(desc) = &task.description {
            println!("   📄 {}", desc);
        }
        
        if let Some(scheduled) = task.scheduled_for {
            println!("   🕐 Programada: {}", scheduled.format("%d/%m/%Y %H:%M"));
        }
        
        if !task.tags.is_empty() {
            println!("   🏷️  Tags: {}", task.tags.join(", "));
        }
        
        println!("   💡 Usa 'snooze {}' para posponer", task.id);
        println!("═══════════════════════════════════\n");
        
        // En sistemas Unix, también podemos usar notify-rust para notificaciones del sistema
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let _ = Command::new("osascript")
                .args(&[
                    "-e",
                    &format!(
                        "display notification \"{}\" with title \"RusTask Reminder\"",
                        task.title
                    ),
                ])
                .output();
        }
    }
}