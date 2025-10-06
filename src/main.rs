use std::io;
use clap::Parser; 

// Declarar nuestros módulos
mod task;
mod storage;
mod cli;

// Imports de nuestros módulos
use task::Task;
use storage::{TaskStorage, TaskStats};
use cli::{Cli, Commands};

fn main() {
    println!("🦀 RusTask - Interactive Mode");
    println!("Type 'exit' to quit\n");
    
    // Crear el storage (por ahora en memoria)
    let mut storage = TaskStorage::new();
    loop {
        println!("\nrustask>");   
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();   

        if input == "exit" || input.is_empty() {
            break;
        }
        let args = parse_args(input);

        let mut full_args = vec!["rustask".to_string()];
        full_args.extend(args);

        match Cli::try_parse_from(full_args) {
          Ok(cli) => {
              // Ejecutar el comando correspondiente
              match cli.command {
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
              }
          },
          Err(e) => {
              println!("❌ Error: {}", e);
          }
        }
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
    let tasks = if completed && !pending {
        // Solo completadas
        storage.get_tasks_by_status(true)
    } else if pending && !completed {
        // Solo pendientes
        storage.get_tasks_by_status(false)
    } else if let Some(tag_filter) = tag {
        // Filtrar por tag
        storage.find_tasks_by_tag(&tag_filter)
    } else {
        // Todas las tareas
        storage.get_all_tasks().iter().collect()
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
fn handle_complete(storage: &mut TaskStorage, id: u32) {
    if storage.complete_task(id) {
        println!("✅ Tarea {} marcada como completada", id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: delete  
fn handle_delete(storage: &mut TaskStorage, id: u32) {
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
fn handle_show(storage: &TaskStorage, id: u32) {
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
fn handle_update(storage: &mut TaskStorage, id: u32, title: Option<String>, description: Option<String>, tags: Vec<String>) {
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
fn handle_add_tag(storage: &mut TaskStorage, id: u32, tag: String) {
    if let Some(task) = storage.find_task_by_id_mut(id) {
        task.add_tag(tag.clone());
        println!("🏷️ Tag '{}' agregado a la tarea {}", tag, id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}

// Manejar comando: remove-tag
fn handle_remove_tag(storage: &mut TaskStorage, id: u32, tag: String) {
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
fn handle_clear_tags(storage: &mut TaskStorage, id: u32) {
    if let Some(task) = storage.find_task_by_id_mut(id) {
        let tags_count = task.get_tags().len();
        task.clear_tags();
        println!("🧹 {} tags removidos de la tarea {}", tags_count, id);
    } else {
        println!("❌ No se encontró una tarea con ID {}", id);
    }
}
