use colored::*;
use crate::task::{load_tasks, save_tasks_safe, Task, get_data_location, get_today, get_date_with_offset, validate_date, load_removed_tasks, add_to_removed};

// Helper function to validate task ID exists
fn validate_task_id(id: u32, tasks: &[Task]) -> Result<usize, String> {
    if id == 0 {
        return Err("Task ID cannot be 0".to_string());
    }
    
    match tasks.iter().position(|t| t.id == id) {
        Some(index) => Ok(index),
        None => Err(format!("Task {} not found", id)),
    }
}

// Helper function to format date with calendar emoji and month abbreviation
fn format_date_with_emoji(date: &str) -> String {
    // Extract month and day from date (YYYY-MM-DD format)
    let parts: Vec<&str> = date.split('-').collect();
    let month = parts.get(1).unwrap_or(&"01");
    let day = parts.get(2).unwrap_or(&"01");
    let day_num: u32 = day.parse().unwrap_or(1);
    
    // Month abbreviations
    let month_emoji = match month.as_ref() {
        "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
        "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
        "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
        _ => "???",
    };

    format!(" {}/{}", day_num.to_string().bright_yellow().bold(), month_emoji.bright_yellow().bold())
}

// Helper function to display a list of tasks grouped by month
fn display_task_list(tasks: &[&Task], header: &str, dimmed: bool, header_color: &str) {
    if tasks.is_empty() {
        return;
    }
    
    // Apply different colors based on header_color parameter
    match header_color {
        "red" => println!("{}", header.red().bold()),
        "blue" => println!("{}", header.blue().bold()),
        _ => println!("{}", header.blue().bold()), // default to blue
    }
    
    // Group tasks by year-month and sort
    use std::collections::BTreeMap;
    let mut grouped_tasks: BTreeMap<String, Vec<&Task>> = BTreeMap::new();
    
    for task in tasks {
        if let Some(due_date) = &task.due_date {
            let parts: Vec<&str> = due_date.split('-').collect();
            let year = parts.get(0).unwrap_or(&"0000");
            let month = parts.get(1).unwrap_or(&"00");
            
            let group_key = format!("{}-{}", year, month); // For sorting
            
            grouped_tasks.entry(group_key).or_insert_with(Vec::new).push(*task);
        } else {
            // Tasks without dates go to a special group
            grouped_tasks.entry("0000-00".to_string()).or_insert_with(Vec::new).push(*task);
        }
    }
    
    // Display each group
    for (group_key, mut group_tasks) in grouped_tasks {
        // Sort tasks within each group by due date, then by id
        group_tasks.sort_by(|a, b| {
            match (&a.due_date, &b.due_date) {
                (Some(date_a), Some(date_b)) => date_a.cmp(date_b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.id.cmp(&b.id),
            }
        });
        
        // Display group header
        if group_key == "0000-00" {
            println!("\n{}", "[No Due Date]".bright_black().bold());
        } else {
            let parts: Vec<&str> = group_key.split('-').collect();
            let year = parts.get(0).unwrap_or(&"0000");
            let month = parts.get(1).unwrap_or(&"00");
            let month_name = match month.as_ref() {
                "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
                "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
                "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
                _ => "???",
            };
            println!("╔══════════════════════════════════╗");
            println!("║          {}", format!("  {} {}              ║", month_name.bright_cyan().bold(), year.bright_cyan().bold()));
            println!("╚══════════════════════════════════╝");
        }
        
        // Display tasks in this group
        for task in group_tasks {
            let status = if task.done { "✅" } else { "⬜" };
            let due_info = match &task.due_date {
                Some(date) => format_date_with_emoji(date),
                None => String::new(),
            };
            
            // Check task status for color coding
            let today = crate::task::get_today();
            let task_text = if !task.done {
                if let Some(due_date) = &task.due_date {
                    if due_date < &today {
                        // Overdue tasks - red
                        task.text.red().to_string()
                    } else if due_date > &today {
                        // Future tasks - check if tomorrow or later
                        let tomorrow = crate::task::get_date_with_offset(1);
                        if due_date == &tomorrow {
                            // Tomorrow tasks - light green
                            task.text.bright_blue().to_string()
                        } else {
                            // Tasks after tomorrow - green
                            task.text.green().to_string()
                        }
                    } else {
                        // Today's tasks - normal color
                        task.text.bold().to_string()
                    }
                } else {
                    // No due date - normal color
                    task.text.to_string()
                }
            } else {
                // Completed tasks - normal color
                task.text.to_string()
            };
            
            let line = format!("{} [{}] {}{}", task.id.to_string().bright_green(), status, task_text, due_info);
            
            if task.done {
                println!("{}", line.strikethrough().dimmed());
            } else if dimmed {
                println!("{}", line.dimmed());
            } else {
                println!("{}", line);
            }
        }
    }
}

// Add new task
pub fn add_task(text: String, days_offset: i32) {
    // Validate input text
    if text.trim().is_empty() {
        println!("{} Task text cannot be empty!", "❌".red());
        return;
    }
    
    if text.len() > 500 {
        println!("{} Task text is too long (max 500 characters)!", "❌".red());
        return;
    }
    
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let due_date = get_date_with_offset(days_offset);
    
    tasks.push(Task { 
        id, 
        text: text.clone(), 
        done: false, 
        due_date: Some(due_date.clone()) 
    });
    
    save_tasks_safe(&tasks);
    
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
    } else {
        // Sort pending tasks by due date
        filtered_tasks.sort_by(|a, b| {
            match (&a.due_date, &b.due_date) {
                (Some(date_a), Some(date_b)) => date_a.cmp(date_b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.id.cmp(&b.id),
            }
        });
    }

    let header = if today_only {
        format!("📅 Today's Tasks ({}):", today)
    } else if show_all {
        "📋 Your Complete Task List:".to_string()
    } else {
        "📋 Your Pending Tasks:".to_string()
    };
    
    display_task_list(&filtered_tasks, &header, false, "blue");
}

// Toggle task completion
pub fn toggle_task(id: u32) {
    let mut tasks = load_tasks();
    
    // Validate task ID
    let index = match validate_task_id(id, &tasks) {
        Ok(idx) => idx,
        Err(err) => {
            println!("{} {}", "❌".red(), err);
            return;
        }
    };

    tasks[index].done = !tasks[index].done;
    let new_done = tasks[index].done;

    save_tasks_safe(&tasks);
    println!("{} Task {} {}", "🎉".green(), id, if new_done { "completed!".green() } else { "reopened!".yellow() });
}

// Delete task
pub fn delete_task(id: u32) {
    let mut tasks = load_tasks();
    
    // Validate task ID
    let _index = match validate_task_id(id, &tasks) {
        Ok(idx) => idx,
        Err(err) => {
            println!("{} {}", "❌".red(), err);
            return;
        }
    };
    
    let old_len = tasks.len();
    tasks.retain(|t| t.id != id);
    
    if tasks.len() < old_len {
        save_tasks_safe(&tasks);
        println!("{} Task {} deleted!", "🗑️".red(), id);
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
        
        save_tasks_safe(&updated_tasks);
        println!("{} Successfully removed {} task(s) {}!", 
            "✅".green(), removed_count, date_desc);
    } else {
        println!("{} Operation cancelled.", "❌".yellow());
    }
}

// Edit task
pub fn edit_task(id: u32, new_text: String) {
    // Validate input text
    if new_text.trim().is_empty() {
        println!("{} Task text cannot be empty!", "❌".red());
        return;
    }
    
    if new_text.len() > 500 {
        println!("{} Task text is too long (max 500 characters)!", "❌".red());
        return;
    }
    
    let mut tasks = load_tasks();
    
    // Validate task ID
    let index = match validate_task_id(id, &tasks) {
        Ok(idx) => idx,
        Err(err) => {
            println!("{} {}", "❌".red(), err);
            return;
        }
    };

    tasks[index].text = new_text.clone();
    save_tasks_safe(&tasks);
    println!("{} Task {} updated!", "✏️".green(), id);
}

// Set due date
pub fn set_due_date(id: u32, date: String) {
    // Validate the date format first
    if let Err(err) = validate_date(&date) {
        println!("{} {}", "❌".red(), err);
        return;
    }
    
    let mut tasks = load_tasks();
    
    // Validate task ID
    let index = match validate_task_id(id, &tasks) {
        Ok(idx) => idx,
        Err(err) => {
            println!("{} {}", "❌".red(), err);
            return;
        }
    };

    tasks[index].due_date = Some(date.clone());
    save_tasks_safe(&tasks);
    println!("{} Due date set for task {}: {}", "📅".green(), id, date.yellow());
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
    // Validate search query
    if query.trim().is_empty() {
        println!("{} Search query cannot be empty!", "❌".red());
        return;
    }
    
    if query.len() > 100 {
        println!("{} Search query is too long (max 100 characters)!", "❌".red());
        return;
    }
    
    let tasks = load_tasks();
    let results: Vec<&Task> = tasks.iter()
        .filter(|task| task.text.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    if results.is_empty() {
        println!("{} No tasks match '{}'!", "🔍".yellow(), query);
    } else {
        display_task_list(&results, "🔍 Search Results:", false, "blue");
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
    
    let task_refs: Vec<&Task> = removed_tasks.iter().collect();
    display_task_list(&task_refs, "🗑️ Removed Tasks:", true, "red"); // Use dimmed style for removed tasks
    
    println!("\n{} Total removed tasks: {}", "📊".blue(), removed_tasks.len().to_string().cyan());
}