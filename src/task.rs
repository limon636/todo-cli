use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use std::error::Error;
use std::fmt;

// Custom error type for task operations
#[derive(Debug)]
pub enum TaskError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    InvalidDate(String),
    InvalidInput(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::IoError(err) => write!(f, "File operation failed: {}", err),
            TaskError::JsonError(err) => write!(f, "JSON operation failed: {}", err),
            TaskError::InvalidDate(date) => write!(f, "Invalid date format: {}", date),
            TaskError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for TaskError {}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> Self {
        TaskError::IoError(err)
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(err: serde_json::Error) -> Self {
        TaskError::JsonError(err)
    }
}

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
fn get_todo_dir() -> Result<PathBuf, TaskError> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| TaskError::InvalidInput("Could not find home directory".to_string()))?;
    
    let todo_dir = PathBuf::from(home_dir).join(".todo");
    
    // Create .todo directory if it doesn't exist
    if !todo_dir.exists() {
        fs::create_dir_all(&todo_dir)?;
    }
    
    Ok(todo_dir)
}

// Get the full path to todos.json
fn get_todos_file_path() -> Result<PathBuf, TaskError> {
    Ok(get_todo_dir()?.join("todos.json"))
}

// Get the full path to removed.json
fn get_removed_file_path() -> Result<PathBuf, TaskError> {
    Ok(get_todo_dir()?.join("removed.json"))
}

// Load tasks from file
pub fn load_tasks() -> Vec<Task> {
    match load_tasks_result() {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Warning: Could not load tasks: {}", err);
            Vec::new()
        }
    }
}

// Load tasks from file with error handling
pub fn load_tasks_result() -> Result<Vec<Task>, TaskError> {
    let file_path = get_todos_file_path()?;
    
    // If file doesn't exist, return empty list
    if !file_path.exists() {
        return Ok(Vec::new());
    }
    
    // Try to validate and load the main file
    match validate_json_file(&file_path) {
        Ok(tasks) => Ok(tasks),
        Err(_) => {
            eprintln!("Warning: Main task file is corrupted, attempting to restore from backup...");
            
            // Try to restore from backup
            if let Ok(()) = restore_from_backup() {
                // Try loading again after restoration
                match validate_json_file(&file_path) {
                    Ok(tasks) => {
                        eprintln!("✅ Successfully restored tasks from backup");
                        Ok(tasks)
                    },
                    Err(e) => {
                        eprintln!("❌ Backup file is also corrupted: {}", e);
                        eprintln!("Creating new empty task list...");
                        Ok(Vec::new())
                    }
                }
            } else {
                eprintln!("❌ No backup available, creating new empty task list...");
                Ok(Vec::new())
            }
        }
    }
}

// Save to file
pub fn save_tasks(tasks: &[Task]) -> Result<(), TaskError> {
    let file_path = get_todos_file_path()?;
    
    // Create backup before saving
    create_backup(&file_path)?;
    
    let json = serde_json::to_string_pretty(tasks)?;
    
    // Validate JSON before writing
    let _: Vec<Task> = serde_json::from_str(&json)?;
    
    fs::write(&file_path, json)?;
    Ok(())
}

// Create backup of existing file
fn create_backup(file_path: &std::path::Path) -> Result<(), TaskError> {
    if file_path.exists() {
        let backup_path = file_path.with_extension("json.backup");
        fs::copy(file_path, backup_path)?;
    }
    Ok(())
}

// Restore from backup if main file is corrupted
pub fn restore_from_backup() -> Result<(), TaskError> {
    let file_path = get_todos_file_path()?;
    let backup_path = file_path.with_extension("json.backup");
    
    if backup_path.exists() {
        fs::copy(&backup_path, &file_path)?;
        println!("✅ Restored from backup file");
        Ok(())
    } else {
        Err(TaskError::InvalidInput("No backup file found".to_string()))
    }
}

// Validate JSON file integrity
pub fn validate_json_file(file_path: &std::path::Path) -> Result<Vec<Task>, TaskError> {
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    // Try to parse JSON
    let tasks: Vec<Task> = serde_json::from_str(&contents)
        .map_err(|e| TaskError::JsonError(e))?;
    
    // Validate each task
    for task in &tasks {
        if task.text.is_empty() {
            return Err(TaskError::InvalidInput(format!("Task {} has empty text", task.id)));
        }
        if task.text.len() > 500 {
            return Err(TaskError::InvalidInput(format!("Task {} text too long", task.id)));
        }
        if let Some(ref due_date) = task.due_date {
            validate_date(due_date)?;
        }
    }
    
    Ok(tasks)
}

// Save to file (fallback version that prints errors)
pub fn save_tasks_safe(tasks: &[Task]) {
    if let Err(err) = save_tasks(tasks) {
        eprintln!("Error: Could not save tasks: {}", err);
    }
}

// Get the path where todo data is stored (for user info)
pub fn get_data_location() -> String {
    match get_todos_file_path() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => "Error: Could not determine data location".to_string(),
    }
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

// Validate date format (YYYY-MM-DD)
pub fn validate_date(date: &str) -> Result<(), TaskError> {
    if date.is_empty() {
        return Err(TaskError::InvalidDate("Date cannot be empty".to_string()));
    }
    
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return Err(TaskError::InvalidDate(format!("Date must be in YYYY-MM-DD format, got: {}", date)));
    }
    
    // Check year (4 digits)
    let year: i32 = parts[0].parse()
        .map_err(|_| TaskError::InvalidDate(format!("Invalid year: {}", parts[0])))?;
    if year < 1900 || year > 9999 {
        return Err(TaskError::InvalidDate(format!("Year must be between 1900 and 9999, got: {}", year)));
    }
    
    // Check month (01-12)
    let month: u32 = parts[1].parse()
        .map_err(|_| TaskError::InvalidDate(format!("Invalid month: {}", parts[1])))?;
    if month < 1 || month > 12 {
        return Err(TaskError::InvalidDate(format!("Month must be between 01 and 12, got: {}", month)));
    }
    
    // Check day (01-31, simplified validation)
    let day: u32 = parts[2].parse()
        .map_err(|_| TaskError::InvalidDate(format!("Invalid day: {}", parts[2])))?;
    if day < 1 || day > 31 {
        return Err(TaskError::InvalidDate(format!("Day must be between 01 and 31, got: {}", day)));
    }
    
    // More thorough validation (days per month)
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            // Leap year check
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        },
        _ => return Err(TaskError::InvalidDate("Invalid month".to_string())),
    };
    
    if day > days_in_month {
        return Err(TaskError::InvalidDate(format!("Day {} is invalid for month {}", day, month)));
    }
    
    Ok(())
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

// Load removed tasks from file
pub fn load_removed_tasks() -> Vec<Task> {
    match load_removed_tasks_result() {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Warning: Could not load removed tasks: {}", err);
            Vec::new()
        }
    }
}

// Load removed tasks from file with error handling
pub fn load_removed_tasks_result() -> Result<Vec<Task>, TaskError> {
    let file_path = get_removed_file_path()?;
    
    // If file doesn't exist, return empty list
    if !file_path.exists() {
        return Ok(Vec::new());
    }
    
    // Try to validate and load the file
    match validate_json_file(&file_path) {
        Ok(tasks) => Ok(tasks),
        Err(_) => {
            eprintln!("Warning: Removed tasks file is corrupted, attempting recovery...");
            
            // Try to restore from backup
            let backup_path = file_path.with_extension("json.backup");
            if backup_path.exists() {
                match fs::copy(&backup_path, &file_path) {
                    Ok(_) => {
                        match validate_json_file(&file_path) {
                            Ok(tasks) => {
                                eprintln!("✅ Successfully restored removed tasks from backup");
                                Ok(tasks)
                            },
                            Err(_) => {
                                eprintln!("❌ Backup file is also corrupted, starting with empty removed list");
                                Ok(Vec::new())
                            }
                        }
                    },
                    Err(_) => {
                        eprintln!("❌ Could not restore from backup, starting with empty removed list");
                        Ok(Vec::new())
                    }
                }
            } else {
                eprintln!("❌ No backup available, starting with empty removed list");
                Ok(Vec::new())
            }
        }
    }
}

// Save removed tasks to file
pub fn save_removed_tasks(tasks: &[Task]) -> Result<(), TaskError> {
    let file_path = get_removed_file_path()?;
    
    // Create backup before saving
    create_backup(&file_path)?;
    
    let json = serde_json::to_string_pretty(tasks)?;
    
    // Validate JSON before writing
    let _: Vec<Task> = serde_json::from_str(&json)?;
    
    fs::write(&file_path, json)?;
    Ok(())
}

// Add tasks to removed storage
pub fn add_to_removed(tasks_to_remove: Vec<Task>) {
    match add_to_removed_result(tasks_to_remove) {
        Ok(()) => {},
        Err(err) => eprintln!("Error: Could not add tasks to removed storage: {}", err),
    }
}

// Add tasks to removed storage with error handling
pub fn add_to_removed_result(tasks_to_remove: Vec<Task>) -> Result<(), TaskError> {
    let mut removed_tasks = load_removed_tasks_result()?;
    removed_tasks.extend(tasks_to_remove);
    save_removed_tasks(&removed_tasks)?;
    Ok(())
}