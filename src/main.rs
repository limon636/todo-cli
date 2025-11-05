use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Read;

// তোমার টুডু আইটেম
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    text: String,
    done: bool,
}

// CLI আর্গুমেন্ট
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "🦀 তোমার সুপারফাস্ট টুডু টুল")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// নতুন টাস্ক যোগ করো
    Add { text: String },
    /// সব টাস্ক দেখাও
    List,
    /// টাস্ক সম্পন্ন করো
    Done { id: u32 },
    /// টাস্ক মুছে ফেলো
    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text } => add_task(text),
        Commands::List => list_tasks(),
        Commands::Done { id } => toggle_task(id),
        Commands::Delete { id } => delete_task(id),
    }
}

// ফাইল থেকে টাস্ক লোড করো
fn load_tasks() -> Vec<Task> {
    let mut file = match OpenOptions::new().read(true).open("todos.json") {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    serde_json::from_str(&contents).unwrap_or_default()
}

// ফাইলে সেভ করো
fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("todos.json", json).expect("ফাইলে লিখতে পারিনি!");
}

// নতুন টাস্ক
fn add_task(text: String) {
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task { id, text, done: false });
    save_tasks(&tasks);
    println!("{} {}", "✅ যোগ হয়েছে!".green(), id.to_string().cyan());
}

// সব দেখাও
fn list_tasks() {
    let tasks = load_tasks();
    if tasks.is_empty() {
        println!("{}", "📭 কোনো টাস্ক নেই! কিছু যোগ করো।".yellow());
        return;
    }

    println!("{}", "📋 তোমার টাস্ক লিস্ট:".blue().bold());
    for task in tasks {
        let status = if task.done { "✅" } else { "⬜" };
        let line = format!("{} [{}] {}", task.id, status, task.text);
        if task.done {
            println!("{}", line.strikethrough().dimmed());
        } else {
            println!("{}", line);
        }
    }
}

// সম্পন্ন করো — **ফিক্সড!**
fn toggle_task(id: u32) {
    let mut tasks = load_tasks();
    let mut found = false;
    let mut new_done = false;

    for task in &mut tasks {
        if task.id == id {
            task.done = !task.done;
            new_done = task.done;
            found = true;
            break;
        }
    }

    if found {
        save_tasks(&tasks);
        println!("{} টাস্ক {} {}", "🎉".green(), id, if new_done { "সম্পন্ন!".green() } else { "আবার চালু!".yellow() });
    } else {
        println!("{} টাস্ক {} পাওয়া যায়নি!", "❌".red(), id);
    }
}

// মুছে ফেলো
fn delete_task(id: u32) {
    let mut tasks = load_tasks();
    let old_len = tasks.len();
    tasks.retain(|t| t.id != id);
    
    if tasks.len() < old_len {
        save_tasks(&tasks);
        println!("{} টাস্ক {} মুছে ফেলা হয়েছে!", "🗑️".red(), id);
    } else {
        println!("{} টাস্ক {} পাওয়া যায়নি!", "❌".red(), id);
    }
}