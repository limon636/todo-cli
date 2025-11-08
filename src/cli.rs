use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "🦀 Your Super Fast Todo Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add { text: String },
    /// Show all tasks
    List,
    /// Mark task as done
    Done { id: u32 },
    /// Delete a task
    Delete { id: u32 },
    /// Edit a task
    Edit { id: u32, text: String },
    /// Set due date for a task
    Due { id: u32, date: String },
    /// Sync to GitHub Gist
    Sync,
    /// Party time!
    Party,
    /// Search tasks
    Search { query: String },
    /// Launch TUI mode
    Tui,
}