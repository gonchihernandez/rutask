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
        
        // Enviar notificación del sistema en macOS
        #[cfg(target_os = "macos")]
        {
            Self::send_macos_notification(task);
        }
    }
    
    // Envía notificaciones nativas de macOS usando terminal-notifier
    #[cfg(target_os = "macos")]
    fn send_macos_notification(task: &crate::task::Task) {
        use std::process::Command;
        
        // Construir el mensaje de la notificacion
        let mut message = task.title.clone();
        if let Some(desc) = &task.description {
            message.push_str("\n");
            message.push_str(desc);
        }
        
        // Ejecutar terminal-notifier para mostrar la notificacion
        let result = Command::new("terminal-notifier")
            .args(&[
                "-title", "🦀 RusTask",
                "-subtitle", "Recordatorio de Tarea",
                "-message", &message,
                "-sound", "Glass",
                "-sender", "com.apple.Terminal",
            ])
            .output();
        
        // Verificar si la notificacion se envió correctamente
        match result {
            Ok(output) if output.status.success() => {
                println!("✅ Notificación enviada");
            }
            Ok(output) => {
                eprintln!("⚠️ Error al enviar notificación: {}", 
                         String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("⚠️ terminal-notifier no disponible: {}", e);
                eprintln!("   Instala con: brew install terminal-notifier");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_notification() {
        use std::process::Command;
        
        println!("🧪 Probando notificación de macOS...");
        
        let result = Command::new("terminal-notifier")
            .args(&[
                "-title", "RusTask Test",
                "-message", "Esta es una notificación de prueba",
                "-sound", "Glass",
                "-sender", "com.apple.Terminal",
            ])
            .output();
        
        match result {
            Ok(output) if output.status.success() => {
                println!("✅ Notificación enviada exitosamente");
                println!("   Deberías ver una notificación en la esquina superior derecha");
            }
            Ok(output) => {
                println!("❌ Error al enviar notificación");
                println!("   stderr: {}", String::from_utf8_lossy(&output.stderr));
                panic!("La notificación falló");
            }
            Err(e) => {
                println!("❌ Error ejecutando terminal-notifier: {}", e);
                println!("   Instala con: brew install terminal-notifier");
                panic!("No se pudo ejecutar terminal-notifier");
            }
        }
    }
}