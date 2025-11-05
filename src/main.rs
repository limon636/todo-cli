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
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<String>,
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
    /// টাস্ক এডিট করো
    Edit { id: u32, text: String },
    /// টাস্কের ডিউ ডেট সেট করো
    Due { id: u32, date: String },
    /// GitHub Gist-এ সিঙ্ক করো
    Sync,
    /// পার্টি করো!
    Party,
}

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
    tasks.push(Task { id, text, done: false, due_date: None });
    save_tasks(&tasks);
    println!("{} {}", "✅ যোগ হয়েছে!".green(), id.to_string().cyan());
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
        let due_info = match &task.due_date {
            Some(date) => format!(" 📅 {}", date.yellow()),
            None => String::new(),
        };
        let line = format!("{} [{}] {}{}", task.id, status, task.text, due_info);
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
// টাস্ক এডিট করো
fn edit_task(id: u32, new_text: String) {
    let mut tasks = load_tasks();
    let mut found = false;

    for task in &mut tasks {
        if task.id == id {
            task.text = new_text.clone();
            found = true;
            break;
        }
    }

    if found {
        save_tasks(&tasks);
        println!("{} টাস্ক {} আপডেট হয়েছে!", "✏️".green(), id);
    } else {
        println!("{} টাস্ক {} পাওয়া যায়নি!", "❌".red(), id);
    }
}

// ডিউ ডেট সেট করো
fn set_due_date(id: u32, date: String) {
    let mut tasks = load_tasks();
    let mut found = false;

    for task in &mut tasks {
        if task.id == id {
            task.due_date = Some(date.clone());
            found = true;
            break;
        }
    }

    if found {
        save_tasks(&tasks);
        println!("{} টাস্ক {} এর ডিউ ডেট সেট হয়েছে: {}", "📅".green(), id, date.yellow());
    } else {
        println!("{} টাস্ক {} পাওয়া যায়নি!", "❌".red(), id);
    }
}

// GitHub Gist-এ সিঙ্ক করো
fn sync_tasks() {
    let tasks = load_tasks();
    let json = serde_json::to_string_pretty(&tasks).unwrap();
    
    println!("{}", "🔄 সিঙ্ক করা হচ্ছে...".cyan());
    println!("{}", "ℹ️  GitHub Gist সিঙ্ক ফিচার আসছে শীঘ্রই!".yellow());
    println!("{}", "📋 বর্তমান টাস্ক ডেটা:".blue());
    println!("{}", json.dimmed());
    println!("\n{}", "💡 টিপস: আপাতত তুমি ম্যানুয়ালি todos.json ফাইলটি Gist-এ আপলোড করতে পারো!".green());
}

// পার্টি করো!
fn party() {
    let confetti = vec!["🎉", "🎊", "🥳", "🎈", "🎆", "✨", "🌟", "💫", "��", "🎁"];
    println!("\n{}", "🎉 পার্টি টাইম! ��".green().bold());
    
    for _ in 0..3 {
        print!("   ");
        for _ in 0..20 {
            let emoji = confetti[rand() % confetti.len()];
            print!("{} ", emoji);
        }
        println!();
    }
    
    println!("\n{}", "  🎊 অসাধারণ কাজ! তুমি দারুণ! 🎊".cyan().bold());
    println!("{}", "  ✨ সব টাস্ক শেষ! এবার আরাম করো! ✨\n".yellow().bold());
}

// সিম্পল র্যান্ডম ফাংশন
fn rand() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (now.as_nanos() % 10) as usize
}
