use clap::{Parser, Subcommand};
use chrono::NaiveDateTime;

#[derive(Parser)]
#[command(name = "rustask")]
#[command(about = "Un gestor de tareas simple desde la terminal")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Crear una nueva tarea
  Add {
      /// Título de la tarea
      title: String,
      
      /// Descripción opcional de la tarea
      #[arg(short, long)]
      description: Option<String>,
      
      /// Tags para la tarea (pueden ser múltiples)
      #[arg(short, long)]
      tags: Vec<String>,
  },
  
  /// Listar todas las tareas
  List {
      /// Mostrar solo tareas completadas
      #[arg(long)]
      completed: bool,
      
      /// Mostrar solo tareas pendientes  
      #[arg(long)]
      pending: bool,
      
      /// Filtrar por tag específico
      #[arg(short, long)]
      tag: Option<String>,
  },
  
  /// Completar una tarea
  Complete {
      /// ID de la tarea a completar
      id: u64,
  },
  
  /// Eliminar una tarea
  Delete {
      /// ID de la tarea a eliminar
      id: u64,
  },
  
  /// Mostrar estadísticas
  Stats,
  
  /// Mostrar detalles de una tarea específica
  Show {
      /// ID de la tarea a mostrar
      id: u64,
  },
  
  /// Actualizar una tarea existente
  Update {
      /// ID de la tarea a actualizar
      id: u64,
      
      /// Nuevo título (opcional)
      #[arg(short, long)]
      title: Option<String>,
      
      /// Nueva descripción (opcional)
      #[arg(short, long)]
      description: Option<String>,
      
      /// Reemplazar todos los tags con estos nuevos
      #[arg(long)]
      tags: Vec<String>,
  },
  
  /// Agregar un tag a una tarea
  AddTag {
      /// ID de la tarea
      id: u64,
      
      /// Tag a agregar
      tag: String,
  },
  
  /// Remover un tag de una tarea
  RemoveTag {
      /// ID de la tarea
      id: u64,
      
      /// Tag a remover
      tag: String,
  },
  
  /// Limpiar todos los tags de una tarea
  ClearTags {
      /// ID de la tarea
      id: u64,
  },

      /// Programar una tarea para una fecha/hora específica
    Schedule {
        /// ID de la tarea
        id: u64,
        
        /// Fecha y hora (formato: "DD/MM/YYYY HH:MM" o "DD/MM/YYYY")
        #[arg(value_parser = parse_datetime)]
        datetime: chrono::DateTime<chrono::Local>,
    },

    /// Posponer un recordatorio
    Snooze {
        /// ID de la tarea
        id: u64,
        
        /// Minutos para posponer (default: 10)
        #[arg(default_value = "10")]
        minutes: i64,
    },

    /// Listar tareas programadas
    Scheduled
}

fn parse_datetime(s: &str) -> Result<chrono::DateTime<chrono::Local>, String> {
    use chrono::{Local, TimeZone};
    
    // Intentar formato con hora
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%d/%m/%Y %H:%M") {
        return Ok(Local.from_local_datetime(&dt).unwrap());
    }
    
    // Intentar formato solo fecha (usar 09:00 por defecto)
    if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%d/%m/%Y") {
        let dt = date.and_hms_opt(9, 0, 0).unwrap();
        return Ok(Local.from_local_datetime(&dt).unwrap());
    }
    
    Err(format!("Formato de fecha inválido. Use: DD/MM/YYYY HH:MM o DD/MM/YYYY"))
}