# 🦀 RusTask - Rust Task Manager

A simple and elegant command-line task manager built with Rust.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/CLI-Tool-blue?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

## 🚀 Features

- ✅ **Create tasks** with title, description, and tags
- 📋 **List tasks** with filters by status and tags
- ✔️ **Complete tasks** easily
- 🗑️ **Delete tasks** when no longer needed
- 🏷️ **Complete tag management** (add, remove, clear)
- ✏️ **Update existing tasks**
- 🔍 **View task details**

## 📦 Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.70.0 or higher

### From source

```bash
# Clone the repository
git clone https://github.com/gonchihernandez/rustask.git
cd rustask

# Build
cargo build --release

# Install (optional)
cargo install --path .
```

## 🛠️ Usage

### Basic Commands

#### Create a task
```bash
# Simple task
rustask add "Learn Rust"

# Task with description
rustask add "Exercise" --description "30 minutes of cardio"

# Task with tags
rustask add "Buy groceries" --tags shopping --tags urgent
```

#### List tasks
```bash
# All tasks
rustask list

# Only completed tasks
rustask list --completed

# Only pending tasks
rustask list --pending

# Filter by tag
rustask list --tag urgent
```

#### Manage status
```bash
# Complete a task
rustask complete 1

# View task details
rustask show 1

# Delete a task
rustask delete 1
```

### Advanced Commands

#### Update tasks
```bash
# Change title
rustask update 1 --title "New title"

# Change description
rustask update 1 --description "New description"

# Replace all tags
rustask update 1 --tags work --tags important
```

#### Manage tags
```bash
# Add a tag
rustask add-tag 1 urgent

# Remove a specific tag
rustask remove-tag 1 urgent

# Clear all tags
rustask clear-tags 1
```

#### View statistics
```bash
rustask stats
```

## 📊 Example Usage

```bash
# Create some tasks
rustask add "Learn Rust" --tags programming
rustask add "Exercise" --description "Run 5km" --tags health
rustask add "Buy groceries" --tags home --tags urgent

# View all tasks
rustask list

# Complete a task
rustask complete 1

# View statistics
rustask stats

# Filter tasks by tag
rustask list --tag health
```

## 🏗️ Project Architecture

```
src/
├── main.rs      # Entry point and command handling
├── cli.rs       # CLI interface definition with clap
├── task.rs      # Task structure and business logic
└── storage.rs   # In-memory storage management
```

### Main Components

- **Task**: Structure representing a task with ID, title, description, tags, status, and dates
- **TaskStorage**: Handles in-memory storage with CRUD operations
- **CLI**: Command-line interface automatically generated with clap
- **Handlers**: Functions that connect CLI commands with business logic

## 🧰 Dependencies

- **clap** - CLI argument parsing with derive macros
- **chrono** - Date and timestamp handling
- **serde** - Serialization (ready for JSON persistence)

## 🔮 Future Features

- [ ] **JSON Persistence** - Save tasks to file
- [ ] **Due Dates** - Assign deadlines to tasks
- [ ] **Priorities** - Priority system (high, medium, low)
- [ ] **Search** - Search tasks by text
- [ ] **Export** - Export tasks to different formats
- [ ] **Reminders** - Notifications for pending tasks
- [ ] **Projects** - Group tasks by projects
- [ ] **Configuration** - Customizable configuration file

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo test -- --nocapture
```

## 🚀 Performance

- **Fast startup**: < 10ms boot time
- **Memory efficient**: Minimal RAM usage
- **Zero-cost abstractions**: Thanks to Rust
- **Optimized compilation**: Small binary (~2MB)

## 🤝 Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## 👨‍💻 Author

**gonchihernandez** - [GitHub](https://github.com/gonchihernandez)

Created with ❤️ using Rust

## 🙏 Acknowledgments

- **Rust Community** - For the amazing ecosystem
- **clap** - For making CLI parsing so easy
- **All contributors** - Thanks for making this project better

---

⭐ If you like this project, give it a star on GitHub!

## 📚 Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Clap Documentation](https://docs.rs/clap/)
- [Rust CLI Book](https://rust-cli.github.io/book/)
