use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: u32, title: String, description: Option<String>) -> Self {
        Self {
            id,
            title,
            description,
            tags: Vec::new(),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
    
    pub fn new_with_tags(id: u32, title: String, description: Option<String>, tags: Vec<String>) -> Self {
        Self {
            id,
            title,
            description,
            tags,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

        // Agregar un tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {  // Evita duplicados
            self.tags.push(tag);
        }
    }

    // Remover un tag
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            true  // Se removió exitosamente
        } else {
            false  // No se encontró el tag
        }
    }

    // Verificar si tiene un tag específico
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    // Obtener todos los tags
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    // Limpiar todos los tags
    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }
}