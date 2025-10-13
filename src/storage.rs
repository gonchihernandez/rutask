use std::fs;
use std::io;
use std::path::Path;
use crate::task::Task;

// Struct helper para estadísticas
#[derive(Debug)]
pub struct TaskStats {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
}

#[derive(Debug)]
pub struct TaskStorage {
    tasks: Vec<Task>,
    next_id: u64,
}

impl TaskStorage {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Obtener todas las tareas
    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    // Agregar una nueva tarea
    pub fn add_task(&mut self, mut task: Task) -> u64 {
        task.id = self.next_id;  // Asignamos el ID automáticamente
        let id = task.id;         // Guardamos el ID para retornarlo
        self.tasks.push(task);    // Agregamos la tarea al vector
        self.next_id += 1;        // Incrementamos el contador para la próxima tarea
        id  // Retorna el ID asignado
    }

    // Encontrar una tarea por ID (retorna una referencia inmutable)
    pub fn find_task_by_id(&self, id: u64) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
    }

    // Encontrar una tarea mutable por ID (para modificarla)
    pub fn find_task_by_id_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    // Eliminar una tarea por ID
    pub fn delete_task(&mut self, id: u64) -> bool {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);  // retain() mantiene solo las tareas que NO coinciden con el ID
        self.tasks.len() < original_len  // true si se eliminó algo
    }

    // Actualizar una tarea reutilizando find_task_by_id_mut
    pub fn update_task(&mut self, updated_task: Task) -> bool {
        if let Some(task) = self.find_task_by_id_mut(updated_task.id) {
            *task = updated_task;  // Desreferenciamos (*) para asignar el nuevo valor
            true
        } else {
            false
        }
    }

    // Completar una tarea (método de conveniencia)
    pub fn complete_task(&mut self, id: u64) -> bool {
        if let Some(task) = self.find_task_by_id_mut(id) {
            task.complete();  // Usa el método complete() de Task
            true
        } else {
            false
        }
    }

    // Obtener tareas por estado
    pub fn get_tasks_by_status(&self, completed: bool) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.is_completed() == completed)
            .collect()
    }

    // Buscar tareas por tag
    pub fn find_tasks_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| task.has_tag(tag))
            .collect()
    }

    // Obtener estadísticas
    pub fn get_stats(&self) -> TaskStats {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.is_completed()).count();
        let pending = total - completed;

        TaskStats {
            total,
            completed,
            pending,
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), io::Error> {
      // 1. Serializar las tareas a JSON (formato pretty para que sea legible)
      let json: String = serde_json::to_string_pretty(&self.tasks)
          .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      
      // 2. Escribir el JSON al archivo
      fs::write(path, json)?;
      Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), io::Error> {
      if !Path::new(path).exists() {
          // Si el archivo no existe, no hay nada que cargar
          return Ok(());
      }
      
      // 1. Leer el contenido del archivo
      let data = fs::read_to_string(path)?;
      
      // 2. Deserializar el JSON a un vector de tareas
      let tasks: Vec<Task> = serde_json::from_str(&data)
          .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      
      // 3. Actualizar el almacenamiento con las tareas cargadas
      // El next_id debe ser mayor que el ID más alto para evitar duplicados
      self.next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
      self.tasks = tasks;
      
      Ok(())
    }
}
