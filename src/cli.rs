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
    Add { 
        text: String,
        /// Set due date (days from today, default: 0 = today)
        #[arg(default_value = "0")]
        days: i32,
    },
    /// Show all tasks
    List {
        /// Show all tasks including completed ones
        #[arg(short, long)]
        all: bool,
        /// Show today's tasks only
        #[arg(short, long)]
        today: bool,
    },
    /// Mark task as done
    Done { id: u32 },
    /// Delete a task
    Delete { id: u32 },
    /// Remove all tasks from a specific date
    Remove {
        /// Days ago from today (default: 0 = today)
        #[arg(default_value = "0")]
        days: i32,
    },
    /// Show removed tasks
    Removed,
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
    /// Show data storage location
    Info,
}