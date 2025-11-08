mod task;
mod commands;
mod tui;
mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use commands::*;
use tui::run_tui;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text } => add_task(text),
        Commands::List => list_tasks(),
        Commands::Done { id } => toggle_task(id),
        Commands::Delete { id } => delete_task(id),
        Commands::Edit { id, text } => edit_task(id, text),
        Commands::Due { id, date } => set_due_date(id, date),
        Commands::Sync => sync_tasks(),
        Commands::Party => party(),
        Commands::Search { query } => search(query),
        Commands::Tui => {
            if let Err(e) = run_tui() {
                eprintln!("Error: {}", e);
            }
        }
    }
}
