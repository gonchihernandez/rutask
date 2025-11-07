# RusTask

A simple task manager CLI built with Rust. Created as a learning project to understand Rust fundamentals.

## What it does

Manages your tasks from the terminal with an interactive prompt. Tasks are automatically saved to and loaded from a JSON file for persistence across sessions.

## Features

- Interactive mode - no need to type commands repeatedly
- Create and manage tasks with titles, descriptions, and tags
- Filter tasks by status (completed/pending) or by tag
- Basic CRUD operations (Create, Read, Update, Delete)
- Built-in quote parsing for multi-word titles
- Task scheduling - set due dates and times for reminders
- Smart notifications - automatic reminders for scheduled tasks
- Snooze functionality - postpone reminders when you're busy
- JSON persistence - tasks are automatically saved and restored

## Getting Started

You'll need [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/gonchihernandez/rustask.git
cd rustask
cargo run
```

## How to use it

Just run `cargo run` and you'll get an interactive prompt:

```
🦀 RusTask - Interactive Mode
Type 'exit' to quit

rustask>
```

### Creating tasks

```bash
# Simple task
add "Learn Rust"

# With description
add "Exercise" --description "30 min run"

# With tags
add "Buy groceries" --tags shopping --tags urgent
```

### Listing tasks

```bash
# Show all
list

# Only completed
list --completed

# Only pending
list --pending

# Filter by tag
list --tag urgent
```

### Managing tasks

```bash
# Mark as done
complete 1

# See details
show 1

# Delete
delete 1

# See stats
stats
```

### Working with tags

```bash
# Add a tag
add-tag 1 important

# Remove a tag
remove-tag 1 important

# Clear all tags
clear-tags 1

# Remove a specific tag
rustask remove-tag 1 urgent

# Clear all tags
rustask clear-tags 1
```

### Update tasks

```bash
# Change title
update 1 --title "New title"

# Change description  
update 1 --description "New description"

# Update tags
update 1 --tags work --tags important
```

### Task scheduling

```bash
# Schedule a task for a specific date and time
schedule 1 "06/11/2025 14:30"

# Schedule for a date (defaults to 9:00 AM)
schedule 1 "07/11/2025"

# View scheduled tasks
scheduled

# Snooze a reminder (postpone for 10 minutes by default)
snooze 1

# Snooze for a custom duration
snooze 1 30
```

## Example session

```
$ cargo run

🦀 RusTask - Interactive Mode
Type 'exit' to quit

rustask>
add "Learn Rust" --tags programming
✅ Tarea creada con ID: 1

rustask>
add "Exercise" --description "Run 5km" --tags health
✅ Tarea creada con ID: 2

rustask>
list
📋 Lista de tareas:

⏳ [1] Learn Rust [programming]
⏳ [2] Exercise [health]
    📄 Run 5km

rustask>
complete 1
✅ Tarea 1 marcada como completada

rustask>
stats
📊 Estadísticas de tareas:
   📝 Total: 2
   ✅ Completadas: 1
   ⏳ Pendientes: 1
   🎯 Progreso: 50.0%

rustask>
exit
```

## Project structure

```
src/
├── main.rs      - Entry point, interactive loop, command parsing
├── cli.rs       - Command definitions using clap
├── task.rs      - Task struct and methods
├── storage.rs   - JSON file persistence and in-memory task storage
└── scheduler.rs - Background task scheduler and reminder system
```

## Tech stack

- **clap** - Command-line argument parsing
- **chrono** - Date/time handling and scheduling
- **serde** - Data serialization for JSON persistence
- **serde_json** - JSON file format for task storage

## What I learned

This project helped me understand:
- Rust ownership and borrowing
- Lifetimes
- Pattern matching
- Result types and error handling
- Iterators and closures
- Writing custom parsers
- Building CLIs with clap
- Working with threads and Arc<Mutex<T>> for shared state
- File I/O and JSON serialization in Rust
- Building background services and schedulers

## Roadmap

Things I might add:
- MCP Server:
  - Expose CLI functionality through MCP API for use from MCP hosts
  - Integration using [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)
- Plugins:
  - Abstract notifications into a `Notifier` trait allowing different notification implementations (macOS notifications, Slack messages, Discord, etc.)
  - This would exercise trait implementation and crate creation patterns


## Contributing

Feel free to open issues or submit PRs. This is a learning project, so suggestions are welcome!

## License

MIT License - see LICENSE file for details.
