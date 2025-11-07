use std::io::{self, Write};
use clap::Parser; 

// Declarar nuestros módulos
mod task;
mod storage;
mod cli;
mod scheduler;

// Imports de nuestros módulos
use std::sync::{Arc, Mutex};
use cli::{Cli, Commands};
use storage::{TaskStorage, TaskStats};
use scheduler::Scheduler;
use task::Task;

const TASKS_FILE: &str = "tasks.json";

fn main() {
    println!("🦀 RusTask - Interactive Mode");
    println!("Type 'exit' to quit\n");
    
    // Crear el storage con Arc<Mutex> para compartirlo con el scheduler
    let storage: Arc<Mutex<TaskStorage>> = Arc::new(Mutex::new(TaskStorage::new()));
    
    // Cargar tareas del archivo al inicio
    {
        let mut storage_lock = storage.lock().unwrap();
        match storage_lock.load_from_file(TASKS_FILE) {
            Ok(_) => {
                let stats = storage_lock.get_stats();
                if stats.total > 0 {
                    println!("📂 Cargadas {} tareas desde {}", stats.total, TASKS_FILE);
                }
            }
            Err(e) => {
                eprintln!("⚠️ Error al cargar tareas: {}", e);
            }
        }
    }
    
    // Iniciar el scheduler
    let scheduler = Scheduler::new(Arc::clone(&storage));
    scheduler.start();
    
    loop {
        print!("\nrustask> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();   

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            // Detener el scheduler
            scheduler.stop();
            
            // Guardar tareas antes de salir
            let storage_lock = storage.lock().unwrap();
            if let Err(e) = storage_lock.save_to_file(TASKS_FILE) {
                eprintln!("⚠️ Error al guardar tareas: {}", e);
            } else {
                println!("\n💾 Tareas guardadas en {}", TASKS_FILE);
            }
            println!("👋 ¡Hasta luego!");
            break;
        }
        
        let args = parse_args(input);
        let mut full_args = vec!["rustask".to_string()];
        full_args.extend(args);

        match Cli::try_parse_from(full_args) {
          Ok(cli) => {
              handle_command(cli.command, &storage);
              
              // Guardar tareas después de cada comando
              let storage_lock = storage.lock().unwrap();
              if let Err(e) = storage_lock.save_to_file(TASKS_FILE) {
                  eprintln!("⚠️ Error al guardar tareas: {}", e);
              }
          },
          Err(e) => {
              eprintln!("{}", e);
          }
        }
    }
}

fn handle_command(command: Commands, storage: &Arc<Mutex<TaskStorage>>) {
    let mut storage = storage.lock().unwrap();

    match command {
        Commands::Add { title, description, tags } => {
            handle_add(&mut storage, title, description, tags);
        },
        Commands::List { completed, pending, tag } => {
            handle_list(&storage, completed, pending, tag);
        },
        Commands::Complete { id } => {
            handle_complete(&mut storage, id);
        },
        Commands::Delete { id } => {
            handle_delete(&mut storage, id);
        },
        Commands::Stats => {
            handle_stats(&storage);
        },
        Commands::Show { id } => {
            handle_show(&storage, id);
        },
        Commands::Update { id, title, description, tags } => {
            handle_update(&mut storage, id, title, description, tags);
        },
        Commands::AddTag { id, tag } => {
            handle_add_tag(&mut storage, id, tag);
        },
        Commands::RemoveTag { id, tag } => {
            handle_remove_tag(&mut storage, id, tag);
        },
        Commands::ClearTags { id } => {
            handle_clear_tags(&mut storage, id);
        },
        Commands::Schedule { id, datetime } => {
            handle_schedule(&mut storage, id, datetime);
        },
        Commands::Snooze { id, minutes } => {
            handle_snooze(&mut storage, id, minutes);
        },
        Commands::Scheduled => {
            handle_scheduled(&storage);
        },
    }
}

// Parser simple de comillas (sin dependencias)
fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                // Toggle estado de comillas
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                // Espacio fuera de comillas = separador
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                // Cualquier otro carácter se agrega al argumento actual
                current_arg.push(c);
            }
        }
    }

    // No olvidar el último argumento
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

// Manejar comando: add
fn handle_add(storage: &mut TaskStorage, title: String, description: Option<String>, tags: Vec<String>) {
    let task = if tags.is_empty() {
        // Sin tags, usar constructor básico
        Task::new(0, title, description)
    } else {
        // Con tags, usar constructor con tags
        Task::new_with_tags(0, title, description, tags)
    };
    
    let id = storage.add_task(task);
    println!("✅ Tarea creada con ID: {}", id);
}

// Manejar comando: list  
fn handle_list(storage: &TaskStorage, completed: bool, pending: bool, tag: Option<String>) {
    let all_tasks = storage.get_all_tasks();
    
    let tasks: Vec<&Task> = if completed && !pending {
        // Solo completadas
        all_tasks.iter().filter(|t| t.is_completed()).collect()
    } else if pending && !completed {
        // Solo pendientes
        all_tasks.iter().filter(|t| !t.is_completed()).collect()
    } else if let Some(ref tag_filter) = tag {
        // Filtrar por tag
        all_tasks.iter().filter(|t| t.has_tag(tag_filter)).collect()
    } else {
        // Todas las tareas
        all_tasks.iter().collect()
    };
    
    if tasks.is_empty() {
        println!("📝 No hay tareas que mostrar");
        return;
    }
    
    println!("📋 Lista de tareas:\n");
    for task in tasks {
        let status = if task.is_completed() { "✅" } else { "⏳" };
        let tags_str = if task.get_tags().is_empty() {
            String::new()
        } else {
            format!(" [{}]", task.get_tags().join(", "))
        };
        
        println!("{} [{}] {}{}", status, task.id, task.title, tags_str);
        
        if let Some(ref desc) = task.description {
            println!("    📄 {}", desc);
        }
    }
}

// Manejar comando: complete
fn handle_complete(storage: &mut TaskStorage, id: u64) {
    if storage.complete_task(id) {
        println!("✅ Tarea {} marcada como completada", id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: delete  
fn handle_delete(storage: &mut TaskStorage, id: u64) {
    if storage.delete_task(id) {
        println!("🗑️ Tarea {} eliminada", id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: stats
fn handle_stats(storage: &TaskStorage) {
    let stats: TaskStats = storage.get_stats();
    
    println!("📊 Estadísticas de tareas:");
    println!("   📝 Total: {}", stats.total);
    println!("   ✅ Completadas: {}", stats.completed);
    println!("   ⏳ Pendientes: {}", stats.pending);
    
    if stats.total > 0 {
        let completion_rate = (stats.completed as f64 / stats.total as f64) * 100.0;
        println!("   🎯 Progreso: {:.1}%", completion_rate);
    }
}

// Manejar comando: show
fn handle_show(storage: &TaskStorage, id: u64) {
    if let Some(task) = storage.find_task_by_id(id) {
        let status = if task.is_completed() { "✅ Completada" } else { "⏳ Pendiente" };
        
        println!("🔍 Detalles de la tarea {}:\n", id);
        println!("   📝 Título: {}", task.title);
        println!("   📊 Estado: {}", status);
        
        if let Some(ref desc) = task.description {
            println!("   📄 Descripción: {}", desc);
        }
        
        if !task.get_tags().is_empty() {
            println!("   🏷️ Tags: {}", task.get_tags().join(", "));
        }
        
        println!("   📅 Creada: {}", task.created_at.format("%Y-%m-%d %H:%M:%S"));
        
        if let Some(completed_at) = task.completed_at {
            println!("   ✅ Completada: {}", completed_at.format("%Y-%m-%d %H:%M:%S"));
        }
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: update
fn handle_update(storage: &mut TaskStorage, id: u64, title: Option<String>, description: Option<String>, tags: Vec<String>) {
    if let Some(mut task) = storage.find_task_by_id(id).cloned() {
        // Actualizar campos si se proporcionan
        if let Some(new_title) = title {
            task.title = new_title;
        }
        
        // Para description, necesitamos manejar el caso especial donde el usuario quiere limpiarla
        if description.is_some() {
            task.description = description;
        }
        
        // Reemplazar tags si se proporcionan
        if !tags.is_empty() {
            task.clear_tags();
            for tag in tags {
                task.add_tag(tag);
            }
        }
        
        if storage.update_task(task) {
            println!("✅ Tarea {} actualizada exitosamente", id);
        } else {
            println!("❌ Error al actualizar la tarea {}", id);
        }
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: add-tag
fn handle_add_tag(storage: &mut TaskStorage, id: u64, tag: String) {
    if let Some(task) = storage.find_task_by_id_mut(id) {
        task.add_tag(tag.clone());
        println!("🏷️ Tag '{}' agregado a la tarea {}", tag, id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: remove-tag
fn handle_remove_tag(storage: &mut TaskStorage, id: u64, tag: String) {
    if let Some(task) = storage.find_task_by_id_mut(id) {
        if task.remove_tag(&tag) {
            println!("🗑️ Tag '{}' removido de la tarea {}", tag, id);
        } else {
            println!("❌ La tarea {} no tiene el tag '{}'", id, tag);
        }
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: clear-tags
fn handle_clear_tags(storage: &mut TaskStorage, id: u64) {
    if let Some(task) = storage.find_task_by_id_mut(id) {
        let tags_count = task.get_tags().len();
        task.clear_tags();
        println!("🧹 {} tags removidos de la tarea {}", tags_count, id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: schedule
fn handle_schedule(storage: &mut TaskStorage, id: u64, datetime: chrono::DateTime<chrono::Local>) {
    if let Some(task) = storage.find_task_by_id_mut(id) {
        task.schedule_for(datetime);
        println!("⏰ Tarea {} programada para {}", id, datetime.format("%d/%m/%Y %H:%M"));
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: snooze
fn handle_snooze(storage: &mut TaskStorage, id: u64, minutes: i64) {
    if storage.snooze_task(id, minutes) {
        println!("⏸️ Tarea {} pospuesta por {} minutos", id, minutes);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: scheduled
fn handle_scheduled(storage: &TaskStorage) {
    let tasks: Vec<_> = storage.get_scheduled_tasks().collect();
    
    if tasks.is_empty() {
        println!("📅 No hay tareas programadas");
    } else {
        println!("📅 Tareas programadas:\n");
        for task in tasks {
            let status = if task.is_completed() { "✅" } else { "⏳" };
            print!("{} [{}] {}", status, task.id, task.title);
            
            if let Some(scheduled) = task.scheduled_for {
                print!(" - 🕐 {}", scheduled.format("%d/%m/%Y %H:%M"));
            }
            
            if let Some(snoozed) = task.snoozed_until {
                print!(" (⏸️ hasta {})", snoozed.format("%H:%M"));
            }
            
            if !task.get_tags().is_empty() {
                print!(" [{}]", task.get_tags().join(", "));
            }
            
            println!();
        }
    }
}
