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

pub use task::{Task, load_tasks, save_tasks, get_data_location, get_today, get_date_with_offset};
pub use commands::*;
pub use tui::{run_tui, App, AppMode};
pub use cli::{Cli, Commands};