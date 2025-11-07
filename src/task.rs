use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, Utc};
use chrono::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub scheduled_for: Option<DateTime<Local>>,
    pub reminder_sent: bool,
    pub snoozed_until: Option<DateTime<Local>>,
    pub snooze_count: u32,
}

impl Task {
    pub fn new(id: u64, title: String, description: Option<String>) -> Self {
        Self {
            id,
            title,
            description,
            tags: Vec::new(),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
            scheduled_for: None,
            reminder_sent: false,
            snoozed_until: None,
            snooze_count: 0,
        }
    }
    
    pub fn new_with_tags(id: u64, title: String, description: Option<String>, tags: Vec<String>) -> Self {
        Self {
            id,
            title,
            description,
            tags,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
            scheduled_for: None,
            reminder_sent: false,
            snoozed_until: None,
            snooze_count: 0,
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

    // Programar un recordatorio
    pub fn schedule_for(&mut self, datetime: DateTime<Local>) {
        self.scheduled_for = Some(datetime);
        self.reminder_sent = false;
    }

    pub fn snooze(&mut self, minutes: i64) {
        self.snoozed_until = Some(Local::now() + Duration::minutes(minutes));
        self.snooze_count += 1;
    }

    pub fn is_due(&self) -> bool {
        if let Some(snoozed) = self.snoozed_until {
            return Local::now() >= snoozed;
        }
        
        if let Some(scheduled) = self.scheduled_for {
            return Local::now() >= scheduled && !self.reminder_sent;
        }
        
        false
    }

    pub fn mark_reminder_sent(&mut self) {
        self.reminder_sent = true;
    }

    pub fn clear_snooze(&mut self) {
        self.snoozed_until = None;
    }
}