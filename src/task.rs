use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Read;

// Your Todo Item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

// Load tasks from file
pub fn load_tasks() -> Vec<Task> {
    let mut file = match OpenOptions::new().read(true).open("todos.json") {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();
    serde_json::from_str(&contents).unwrap_or_default()
}

// Save to file
pub fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("todos.json", json).expect("Could not write to file!");
}