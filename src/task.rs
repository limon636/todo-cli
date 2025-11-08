use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::PathBuf;

// Your Todo Item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

// Get the todo data directory path
fn get_todo_dir() -> PathBuf {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("Could not find home directory");
    
    let todo_dir = PathBuf::from(home_dir).join(".todo");
    
    // Create .todo directory if it doesn't exist
    if !todo_dir.exists() {
        fs::create_dir_all(&todo_dir).expect("Could not create .todo directory");
    }
    
    todo_dir
}

// Get the full path to todos.json
fn get_todos_file_path() -> PathBuf {
    get_todo_dir().join("todos.json")
}

// Load tasks from file
pub fn load_tasks() -> Vec<Task> {
    let file_path = get_todos_file_path();
    
    let mut file = match OpenOptions::new().read(true).open(&file_path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    serde_json::from_str(&contents).unwrap_or_default()
}

// Save to file
pub fn save_tasks(tasks: &[Task]) {
    let file_path = get_todos_file_path();
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(&file_path, json).expect("Could not write to file!");
}

// Get the path where todo data is stored (for user info)
pub fn get_data_location() -> String {
    get_todos_file_path().to_string_lossy().to_string()
}

// Get today's date in YYYY-MM-DD format
pub fn get_today() -> String {
    use std::process::Command;
    
    // Use system date command for accurate date
    let output = Command::new("date")
        .arg("+%Y-%m-%d")
        .output();
    
    match output {
        Ok(output) => {
            String::from_utf8(output.stdout)
                .unwrap_or_else(|_| "2025-11-07".to_string())
                .trim()
                .to_string()
        },
        Err(_) => {
            // Fallback to a simple implementation
            "2025-11-07".to_string()
        }
    }
}

// Get date with offset days from today in YYYY-MM-DD format
pub fn get_date_with_offset(days: i32) -> String {
    use std::process::Command;
    
    let offset_arg = if days >= 0 {
        format!("+{} days", days)
    } else {
        format!("{} days", days)
    };
    
    // Use system date command with offset
    let output = Command::new("date")
        .arg("-d")
        .arg(&offset_arg)
        .arg("+%Y-%m-%d")
        .output();
    
    match output {
        Ok(output) => {
            String::from_utf8(output.stdout)
                .unwrap_or_else(|_| get_today())
                .trim()
                .to_string()
        },
        Err(_) => {
            // Fallback to today's date
            get_today()
        }
    }
}