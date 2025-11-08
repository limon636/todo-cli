mod task;
mod commands;
mod tui;
mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use commands::*;
use tui::run_tui;
use task::restore_from_backup;
use colored::*;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text, days } => add_task(text, days),
        Commands::List { all, today } => list_tasks(all, today),
        Commands::Done { id } => toggle_task(id),
        Commands::Delete { id } => delete_task(id),
        Commands::Remove { days } => remove_tasks_by_date(days),
        Commands::Removed => show_removed_tasks(),
        Commands::Edit { id, text } => edit_task(id, text),
        Commands::Due { id, date } => set_due_date(id, date),
        Commands::Sync => sync_tasks(),
        Commands::Party => party(),
        Commands::Search { query } => search(query),
        Commands::Info => show_info(),
        Commands::Restore => {
            match restore_from_backup() {
                Ok(_) => println!("{} Tasks restored from backup successfully!", "✅".green()),
                Err(err) => println!("{} Failed to restore from backup: {}", "❌".red(), err),
            }
        }
        Commands::Tui => {
            if let Err(e) = run_tui() {
                eprintln!("Error: {}", e);
            }
        }
    }
}
