use clap::{Parser, Subcommand};

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
      id: u32,
  },
  
  /// Eliminar una tarea
  Delete {
      /// ID de la tarea a eliminar
      id: u32,
  },
  
  /// Mostrar estadísticas
  Stats,
  
  /// Mostrar detalles de una tarea específica
  Show {
      /// ID de la tarea a mostrar
      id: u32,
  },
}
