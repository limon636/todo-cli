//! # Todo CLI
//! 
//! A super fast todo list CLI tool written in Rust.
//! 
//! ## Features
//! - CLI mode (command line)
//! - TUI mode (Interactive UI)
//! - JSON-based storage
//! - Due date support
//! - Search feature
//! - Colorful UI

pub mod task;
pub mod commands;
pub mod tui;
pub mod cli;

pub use task::{Task, TaskError, load_tasks, save_tasks, save_tasks_safe, get_data_location, get_today, get_date_with_offset, validate_date, load_removed_tasks, add_to_removed, restore_from_backup};
pub use commands::*;
pub use tui::{run_tui, App, AppMode};
pub use cli::{Cli, Commands};