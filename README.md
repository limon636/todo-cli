# 🦀 Todo CLI – Super Fast Todo Tool in Rust  

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/github/license/limon636/todo-cli?style=for-the-badge)
![Stars](https://img.shields.io/github/stars/limon636/todo-cli?style=social)

> **"Add task in 1 second, list, delete—everything with colors + emojis + JSON save!"**  

A **minimal, fast, beautiful** CLI tool written in Rust.  
No database needed. Just the `todo` command!

---

## ✨ Features List
- ✅ `todo add "task"` → Add new task
- ✅ `todo list` → Show all tasks (strikethrough + emojis)
- ✅ `todo done 1` → Toggle task completion
- ✅ `todo delete 2` → Delete task
- ✅ `todo edit 1 "new text"` → Edit task
- ✅ `todo due 1 "2025-12-31"` → Set due date
- ✅ `todo search "keyword"` → Search tasks
- ✅ `todo party` → Party with confetti! 🎉
- ✅ `todo tui` → **Interactive TUI mode** (ratatui + crossterm)
- 🔄 `todo sync` → GitHub Gist sync (coming soon!)
- 💾 Data saved to `todos.json`
- 🎨 Colors, emojis, error handling

---
## 🚀 Install (30 seconds)

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Clone Project
```bash
git clone https://github.com/YOUR_USERNAME/todo.git
cd todo
```

### 3. Global Install (run from anywhere)
```bash
cargo install --path .
```

### 🎮 Usage

#### CLI Mode (Command Line)
```bash
# Add new task
todo add "Learn Rust"

# List tasks
todo list

# Mark as done
todo done 1

# Delete task
todo delete 1

# Edit task
todo edit 1 "New text"

# Set due date
todo due 1 "2025-12-31"

# Search tasks
todo search "Rust"

# Sync to GitHub Gist (coming soon!)
todo sync

# Party! 🎉
todo party

# Help
todo --help
```

#### 🖥️ TUI Mode (Interactive UI)
```bash
# Launch TUI mode
todo tui
```

**TUI Key Bindings:**
- `q` - Quit application
- `j` / `↓` - Move down
- `k` / `↑` - Move up
- `Space` / `Enter` - Toggle task completion
- `a` - Add new task
- `e` - Edit selected task
- `t` - Set due date
- `d` - Delete task
- `Esc` - Exit input mode

### Example Output
```text
📋 Your Task List:
1 [✅] ~~Learn Rust~~
2 [⬜] Call mother 📅 2025-12-31
```

---

## 🖼️ TUI Mode Screenshot

```
┌🦀 Todo TUI - Your Super Fast Todo Tool──────────────────────────────────┐
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
┌📋 Task List─────────────────────────────────────────────────────────────┐
│>> 1 [⬜] Learn Rust 📅 2025-12-31                                      │
│   2 [✅] Call mother                                                   │
│   3 [⬜] Finish project                                                │
└────────────────────────────────────────────────────────────────────────┘
┌Input───────────────────────────────────────────────────────────────────┐
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
┌💡 Help─────────────────────────────────────────────────────────────────┐
│q: quit | j/k: up/down | Space/Enter: toggle | a: add                  │
│e: edit | t: due date | d: delete                                      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 📁 Project Structure

```
src/
├── main.rs      # Main entry point
├── lib.rs       # Library exports
├── task.rs      # Task struct & file operations  
├── commands.rs  # CLI command implementations
├── tui.rs       # Terminal UI (ratatui)
└── cli.rs       # CLI argument parsing (clap)
```

### 🗂️ Module Organization

**`task.rs`** - Core Data & Storage
- `Task` struct definition
- `load_tasks()` - Load from JSON file
- `save_tasks()` - Save to JSON file

**`commands.rs`** - CLI Commands
- `add_task()` - Add new task
- `list_tasks()` - Show task list
- `toggle_task()` - Toggle completion
- `delete_task()` - Delete task
- `edit_task()` - Edit task
- `set_due_date()` - Set due date
- `search()` - Search tasks
- `sync_tasks()` - Sync (placeholder)
- `party()` - Party! 🎉

**`tui.rs`** - Terminal User Interface
- `App` struct - TUI state management
- `AppMode` enum - Normal/Adding/Editing modes
- `run_tui()` - TUI main loop
- `ui()` - UI rendering function

**`cli.rs`** - Command Line Interface
- `Cli` struct - CLI configuration
- `Commands` enum - Available commands

**`main.rs`** - Entry Point
- Module imports
- CLI parsing & command dispatch

**`lib.rs`** - Library Interface
- Public API exports
- Documentation

### 🎯 Benefits of This Structure

✅ **Modularity** - Each feature in separate file  
✅ **Reusability** - Can be used as a library  
✅ **Maintainability** - Easy to find & edit  
✅ **Testability** - Each module can be tested separately  
✅ **Clean Code** - Clear separation of concerns

---

### 🛠 Developer? Contribute!
```bash
git clone https://github.com/YOUR_USERNAME/todo.git
cd todo
cargo run -- add "New feature"
```

### Future Ideas
- 🔄 GitHub/Gist API integration (full sync feature)
- 📊 `todo stats` → Show statistics
- 🏷️ `todo tag 1 "urgent"` → Add tags
- ⏰ `todo remind 1` → Set reminders
- 🎨 TUI theme customization
- 📁 Multiple todo file support

### 📦 Publish to crates.io
```bash
cargo publish
```

### 🔥 Performance
```text
Compile → 2MB binary
Run → 0.001 seconds
Memory → 5MB
```

### 👨‍💻 Author
Md. Shariful Islam
```text
GitHub: limon636
Email: limon.pstu@gmail.com
```

### ⭐ Support
```text
Star if you like it!
Open issues for questions
```

### Your first Rust CLI is ready!
```bash
todo add "Star on GitHub 🌟"
```

#### Made with ❤️ in Rust
