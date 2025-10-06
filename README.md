# RusTask

A simple task manager CLI built with Rust. Created as a learning project to understand Rust fundamentals.

## What it does

Manages your tasks from the terminal with an interactive prompt. Tasks are stored in memory during the session - no files or databases needed (for now).

## Features

- Interactive mode - no need to type commands repeatedly
- Create and manage tasks with titles, descriptions, and tags
- Filter tasks by status (completed/pending) or by tag
- Basic CRUD operations (Create, Read, Update, Delete)
- Built-in quote parsing for multi-word titles

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
└── storage.rs   - In-memory task storage
```

## Tech stack

- **clap** - Command-line argument parsing
- **chrono** - Date/time handling  
- **serde** - Data serialization (prepared for future persistence)

## What I learned

This project helped me understand:
- Rust ownership and borrowing
- Pattern matching
- Result types and error handling
- Iterators and closures
- Writing custom parsers
- Building CLIs with clap

## Known limitations

- Tasks only exist in memory (lost when you exit)
- No file persistence yet
- Single user only
- No task editing UI (must use commands)

## Roadmap

Things I might add:
- Save/load tasks from JSON file
- Due dates and priorities
- Task search
- Better error messages
- Configuration file support

## Contributing

Feel free to open issues or submit PRs. This is a learning project, so suggestions are welcome!

## License

MIT License - see LICENSE file for details.
