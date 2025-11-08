# 🦀 Todo CLI – Super Fast Todo Tool in Rust  

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/github/license/limon636/todo-cli?style=for-the-badge)
![Stars](https://img.shields.io/github/stars/limon636/todo-cli?style=social)

> **"Add task in 1 second, list, delete—everything with colors + emojis + JSON save!"**  

A **minimal, fast, beautiful** CLI tool written in Rust.  
No database needed. Just the `todo` command!

---

## ✨ Features List
- ✅ `todo add "task"` → Add new task (due today by default)
- ✅ `todo add "task" 3` → Add task due in 3 days
- ✅ `todo list` → Show pending tasks only
- ✅ `todo list -a` → Show all tasks (completed + pending)
- ✅ `todo list -t` → Show today's tasks only
- ✅ `todo done 1` → Toggle task completion
- ✅ `todo delete 2` → Delete task
- ✅ `todo remove` → Remove all tasks from today (with confirmation)
- ✅ `todo remove 7` → Remove all tasks from next week
- ✅ `todo edit 1 "new text"` → Edit task
- ✅ `todo due 1 "2025-12-31"` → Set due date
- ✅ `todo search "keyword"` → Search tasks
- ✅ `todo info` → Show data location & statistics
- ✅ `todo party` → Party with confetti! 🎉
- ✅ `todo tui` → **Interactive TUI mode** (ratatui + crossterm)
- 🔄 `todo sync` → GitHub Gist sync (coming soon!)
- 💾 **Data saved to `~/.todo/todos.json`** (production ready!)
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
git clone https://github.com/limon636/todo-cli.git
cd todo-cli
```

### 3. Global Install (run from anywhere)
```bash
cargo install --path .
```

### 🎮 Usage

#### CLI Mode (Command Line)
```bash
# Add new task (due today by default)
todo add "Learn Rust"

# Add task due tomorrow
todo add "Call client" 1

# Add task due in 7 days
todo add "Weekly review" 7

# Add task due yesterday (past due)
todo add "Missed deadline" -- -1

# List pending tasks only
todo list

# List all tasks (including completed)
todo list -a
# or
todo list --all

# List today's tasks only (undone first, then done)
todo list -t
# or
todo list --today

# Mark as done
todo done 1

# Delete task
todo delete 1

# Remove all previous tasks up to today (with confirmation)
todo remove

# Remove all tasks up to yesterday
todo remove 1

# Remove all tasks from last week
todo remove 7

# Edit task
todo edit 1 "New text"

# Set due date
todo due 1 "2025-12-31"

# Search tasks
todo search "Rust"

# Show data location and statistics
todo info

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
# Adding tasks with different due dates
✅ Added! 1 (due today: 2025-11-08)
✅ Added! 2 (due tomorrow: 2025-11-09)  
✅ Added! 3 (due in 7 days: 2025-11-15)
✅ Added! 4 (due yesterday: 2025-11-07)

# Simple syntax examples
todo add "Finish project"           # Due today
todo add "Team meeting" 1           # Due tomorrow  
todo add "Review code" 3            # Due in 3 days
todo add "Weekly standup" 7         # Due next week
todo add "Overdue task" -- -2       # Due 2 days ago

# Pending tasks only (default)
📋 Your Pending Tasks:
1 [⬜] Learn Rust 📅 2025-11-08
2 [⬜] Call client 📅 2025-11-09

# Remove command with confirmation
🗑️ Tasks to be removed from today (2025-11-08):
  1 [⬜] Learn Rust
  3 [✅] ~~Morning workout~~
  5 [⬜] Team meeting

❓ Do you want to remove all 3 task(s) from today (2025-11-08)? [y/N]: y
✅ Successfully removed 3 task(s) from today (2025-11-08)!

# All tasks (with -a flag) - undone first
📋 Your Complete Task List:
1 [⬜] Learn Rust 📅 2025-11-08
2 [⬜] Call client 📅 2025-11-09
3 [⬜] Weekly review 📅 2025-11-15
5 [✅] ~~Completed task~~ 📅 2025-11-07

# Today's tasks only (with -t flag) - undone first
📅 Today's Tasks (2025-11-08):
1 [⬜] Learn Rust 📅 2025-11-08
6 [⬜] Important meeting 📅 2025-11-08
4 [✅] ~~Morning workout~~ 📅 2025-11-08

# When all tasks are completed
🎉 All tasks completed! Use 'todo list -a' to see completed tasks.

# When no tasks due today
📅 No tasks due today (2025-11-08)!

# Info command output
📊 Todo CLI Information
📁 Data stored at: /home/user/.todo/todos.json
📋 Total tasks: 5
✅ Completed: 2
⬜ Pending: 3
```

---

## 🖼️ TUI Mode Screenshot

```
┌🦀 Todo TUI - Your Super Fast Todo Tool──────────────────────────────────┐
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
┌📋 Task List─────────────────────────────────────────────────────────────┐
│>> 1 [⬜] Learn Rust 📅 2025-12-31                                       │
│   2 [✅] Call mother                                                    │
│   3 [⬜] Finish project                                                 │
└─────────────────────────────────────────────────────────────────────────┘
┌Input────────────────────────────────────────────────────────────────────┐
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
┌💡 Help──────────────────────────────────────────────────────────────────┐
│q: quit | j/k: up/down | Space/Enter: toggle | a: add                    │
│e: edit | t: due date | d: delete                                        │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 📁 Production-Ready Data Storage

Your todo data is stored in a hidden directory `~/.todo/todos.json` in your home directory. This ensures:

✅ **Global Access** - Works from any directory  
✅ **User-Specific** - Each user has their own data  
✅ **Hidden & Organized** - Doesn't clutter your workspace  
✅ **Persistent** - Data survives app updates  

```bash
# Check where your data is stored
todo info

# Your data location
~/.todo/todos.json
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
git clone https://github.com/limon636/todo-cli.git
cd todo-cli
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
