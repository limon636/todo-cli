use colored::*;
use crate::task::{load_tasks, save_tasks, Task, get_data_location, get_today, get_date_with_offset, load_removed_tasks, add_to_removed};

// Add new task
pub fn add_task(text: String, days_offset: i32) {
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let due_date = get_date_with_offset(days_offset);
    
    tasks.push(Task { 
        id, 
        text: text.clone(), 
        done: false, 
        due_date: Some(due_date.clone()) 
    });
    
    save_tasks(&tasks);
    
    let date_info = if days_offset == 0 {
        format!(" (due today: {})", due_date.yellow())
    } else if days_offset == 1 {
        format!(" (due tomorrow: {})", due_date.yellow())
    } else if days_offset > 0 {
        format!(" (due in {} days: {})", days_offset, due_date.yellow())
    } else if days_offset == -1 {
        format!(" (due yesterday: {})", due_date.yellow())
    } else {
        format!(" (due {} days ago: {})", days_offset.abs(), due_date.yellow())
    };
    
    println!("{} {}{}", "✅ Added!".green(), id.to_string().cyan(), date_info);
}

// Show all tasks
pub fn list_tasks(show_all: bool, today_only: bool) {
    let tasks = load_tasks();
    if tasks.is_empty() {
        println!("{}", "📭 No tasks! Add some tasks.".yellow());
        return;
    }

    let today = get_today();

    // Filter tasks based on parameters
    let mut filtered_tasks: Vec<&Task> = if today_only {
        // Show today's tasks (with due_date = today)
        tasks.iter()
            .filter(|task| task.due_date.as_ref() == Some(&today))
            .collect()
    } else if show_all {
        // Show all tasks
        tasks.iter().collect()
    } else {
        // Show only pending tasks
        tasks.iter().filter(|task| !task.done).collect()
    };

    if filtered_tasks.is_empty() {
        if today_only {
            println!("{} No tasks due today ({})!", "📅".yellow(), today.cyan());
        } else if show_all {
            println!("{}", "📭 No tasks found.".yellow());
        } else {
            println!("{}", "🎉 All tasks completed! Use 'todo list -a' to see completed tasks.".green());
        }
        return;
    }

    // Sort tasks: undone first, then done (for both -a and --today)
    if show_all || today_only {
        filtered_tasks.sort_by_key(|task| task.done);
    }

    let header = if today_only {
        format!("📅 Today's Tasks ({}):", today)
    } else if show_all {
        "📋 Your Complete Task List:".to_string()
    } else {
        "📋 Your Pending Tasks:".to_string()
    };
    
    println!("{}", header.blue().bold());
    
    for task in filtered_tasks {
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

// Toggle task completion
pub fn toggle_task(id: u32) {
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
        println!("{} Task {} {}", "🎉".green(), id, if new_done { "completed!".green() } else { "reopened!".yellow() });
    } else {
        println!("{} Task {} not found!", "❌".red(), id);
    }
}

// Delete task
pub fn delete_task(id: u32) {
    let mut tasks = load_tasks();
    let old_len = tasks.len();
    tasks.retain(|t| t.id != id);
    
    if tasks.len() < old_len {
        save_tasks(&tasks);
        println!("{} Task {} deleted!", "🗑️".red(), id);
    } else {
        println!("{} Task {} not found!", "❌".red(), id);
    }
}

// Remove all tasks older than specified number of days ago
pub fn remove_tasks_by_date(days_ago: i32) {
    use std::io::{self, Write};
    
    let tasks = load_tasks();
    // Calculate cutoff date: remove tasks older than this date
    // days_ago=1 means remove tasks older than 1 day ago (2+ days old)
    // days_ago=3 means remove tasks older than 3 days ago (4+ days old)
    let cutoff_days = days_ago + 1;
    let cutoff_date = get_date_with_offset(-cutoff_days);
    
    // Find tasks older than cutoff date (due_date < cutoff_date)
    let matching_tasks: Vec<&Task> = tasks.iter()
        .filter(|task| {
            if let Some(due_date) = &task.due_date {
                due_date < &cutoff_date
            } else {
                false
            }
        })
        .collect();
    
    if matching_tasks.is_empty() {
        let date_desc = if days_ago == 0 {
            "older than today".to_string()
        } else {
            format!("older than {}", cutoff_date)
        };
        
        println!("{} No tasks found {}!", "📅".yellow(), date_desc);
        return;
    }
    
    // Show confirmation prompt
    let date_desc = if days_ago == 0 {
        "older than today".to_string()
    } else if days_ago == 1 {
        "older than 1 day ago".to_string()
    } else {
        format!("older than {} days ago", days_ago)
    };
    
    println!("\n{} Tasks to be removed ({}):", "🗑️".red().bold(), date_desc.cyan());
    for task in &matching_tasks {
        let status = if task.done { "✅" } else { "⬜" };
        let due_str = task.due_date.as_ref().map_or("No date".to_string(), |d| d.clone());
        let line = format!("{} [{}] {} ({})", task.id, status, task.text, due_str);
        if task.done {
            println!("  {}", line.strikethrough().dimmed());
        } else {
            println!("  {}", line);
        }
    }
    
    print!("\n{} Do you want to remove all {} task(s) {}? [y/N]: ", 
        "❓".yellow(), matching_tasks.len(), date_desc);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    
    if input == "y" || input == "yes" {
        // Collect tasks to be removed (older than cutoff date)
        let tasks_to_remove: Vec<Task> = tasks.iter()
            .filter(|task| {
                if let Some(due_date) = &task.due_date {
                    due_date < &cutoff_date
                } else {
                    false
                }
            })
            .cloned()
            .collect();
        
        // Add to removed storage
        add_to_removed(tasks_to_remove);
        
        // Remove from active tasks
        let mut updated_tasks = tasks;
        let original_count = updated_tasks.len();
        updated_tasks.retain(|task| {
            if let Some(due_date) = &task.due_date {
                due_date >= &cutoff_date
            } else {
                true // Keep tasks without due dates
            }
        });
        let removed_count = original_count - updated_tasks.len();
        
        save_tasks(&updated_tasks);
        println!("{} Successfully removed {} task(s) {}!", 
            "✅".green(), removed_count, date_desc);
    } else {
        println!("{} Operation cancelled.", "❌".yellow());
    }
}

// Edit task
pub fn edit_task(id: u32, new_text: String) {
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
        println!("{} Task {} updated!", "✏️".green(), id);
    } else {
        println!("{} Task {} not found!", "❌".red(), id);
    }
}

// Set due date
pub fn set_due_date(id: u32, date: String) {
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
        println!("{} Due date set for task {}: {}", "📅".green(), id, date.yellow());
    } else {
        println!("{} Task {} not found!", "❌".red(), id);
    }
}

// Sync to GitHub Gist
pub fn sync_tasks() {
    let tasks = load_tasks();
    let json = serde_json::to_string_pretty(&tasks).unwrap();
    
    println!("{}", "🔄 Syncing...".cyan());
    println!("{}", "ℹ️  GitHub Gist sync feature coming soon!".yellow());
    println!("{}", "📋 Current task data:".blue());
    println!("{}", json.dimmed());
    println!("\n{}", "💡 Tip: For now, you can manually upload the todos.json file to Gist!".green());
}

// Party time!
pub fn party() {
    let confetti = vec!["🎉", "🎊", "🥳", "🎈", "🎆", "✨", "🌟", "💫", "🎇", "🎁"];
    println!("\n{}", "🎉 Party Time! 🎉".green().bold());
    
    for _ in 0..3 {
        print!("   ");
        for _ in 0..20 {
            let emoji = confetti[rand() % confetti.len()];
            print!("{} ", emoji);
        }
        println!();
    }
    
    println!("\n{}", "  🎊 Awesome work! You're amazing! 🎊".cyan().bold());
    println!("{}", "  ✨ All tasks done! Time to relax! ✨\n".yellow().bold());
}

// Search tasks
pub fn search(query: String) {
    let tasks = load_tasks();
    let results: Vec<&Task> = tasks.iter()
        .filter(|task| task.text.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    if results.is_empty() {
        println!("{} No tasks match '{}'!", "🔍".yellow(), query);
    } else {
        println!("{}", "🔍 Search Results:".blue().bold());
        for task in results {
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
}

// Simple random function
fn rand() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (now.as_nanos() % 10) as usize
}

// Show information about data storage location
pub fn show_info() {
    let data_location = get_data_location();
    let tasks = load_tasks();
    
    println!("{}", "📊 Todo CLI Information".blue().bold());
    println!("{} {}", "📁 Data stored at:".green(), data_location.cyan());
    println!("{} {}", "📋 Total tasks:".green(), tasks.len().to_string().cyan());
    println!("{} {}", "✅ Completed:".green(), tasks.iter().filter(|t| t.done).count().to_string().cyan());
    println!("{} {}", "⬜ Pending:".green(), tasks.iter().filter(|t| !t.done).count().to_string().cyan());
}

// Show removed tasks
pub fn show_removed_tasks() {
    let removed_tasks = load_removed_tasks();
    
    if removed_tasks.is_empty() {
        println!("{}", "🗑️ No removed tasks found.".yellow());
        return;
    }
    
    println!("{}", "🗑️ Removed Tasks:".red().bold());
    for task in &removed_tasks {
        let status = if task.done { "✅" } else { "⬜" };
        let due_info = match &task.due_date {
            Some(date) => format!(" 📅 {}", date.yellow()),
            None => String::new(),
        };
        let line = format!("{} [{}] {}{}", task.id, status, task.text, due_info);
        if task.done {
            println!("{}", line.strikethrough().dimmed());
        } else {
            println!("{}", line.dimmed());
        }
    }
    
    println!("\n{} Total removed tasks: {}", "📊".blue(), removed_tasks.len().to_string().cyan());
}