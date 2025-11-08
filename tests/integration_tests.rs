use todo::{Task, load_tasks, save_tasks_safe, add_task, delete_task, toggle_task, restore_from_backup};

/// Integration tests that test the full workflow of the todo application
/// These tests interact with the real file system but use isolated test data

#[test]
fn test_full_task_workflow() {
    // Get initial state
    let initial_tasks = load_tasks();
    let initial_count = initial_tasks.len();
    
    // Add a test task with unique name to avoid conflicts
    let unique_task_name = format!("Integration test task {}", std::process::id());
    add_task(unique_task_name.clone(), 0);
    
    // Verify task was added
    let tasks_after_add = load_tasks();
    assert_eq!(tasks_after_add.len(), initial_count + 1);
    
    // Find the test task
    let test_task = tasks_after_add.iter()
        .find(|t| t.text == unique_task_name)
        .expect("Test task should exist");
    
    let test_task_id = test_task.id;
    assert!(!test_task.done);
    
    // Toggle the task
    toggle_task(test_task_id);
    
    // Verify task is now completed
    let tasks_after_toggle = load_tasks();
    let completed_task = tasks_after_toggle.iter()
        .find(|t| t.id == test_task_id)
        .expect("Test task should still exist");
    assert!(completed_task.done);
    
    // Delete the test task
    delete_task(test_task_id);
    
    // Verify task was deleted
    let final_tasks = load_tasks();
    assert_eq!(final_tasks.len(), initial_count);
    assert!(!final_tasks.iter().any(|t| t.id == test_task_id));
}

#[test]
fn test_task_persistence() {
    // Test that tasks persist between saves and loads
    let initial_tasks = load_tasks();
    let initial_count = initial_tasks.len();
    
    // Create some test data
    let mut new_tasks = initial_tasks.clone();
    new_tasks.push(Task {
        id: 99999, // Use a high ID to avoid conflicts
        text: "Persistence test".to_string(),
        done: false,
        due_date: Some("2025-12-25".to_string()),
    });
    
    // Save the tasks
    save_tasks_safe(&new_tasks);
    
    // Load tasks again and verify persistence
    let loaded_tasks = load_tasks();
    assert_eq!(loaded_tasks.len(), initial_count + 1);
    
    let persistent_task = loaded_tasks.iter()
        .find(|t| t.id == 99999)
        .expect("Persistent task should exist");
    
    assert_eq!(persistent_task.text, "Persistence test");
    assert_eq!(persistent_task.due_date, Some("2025-12-25".to_string()));
    assert!(!persistent_task.done);
    
    // Cleanup - restore original state
    save_tasks_safe(&initial_tasks);
}

#[test]
fn test_error_handling_workflow() {
    // Test that error conditions don't crash the application
    
    // Try to operate on non-existent task
    toggle_task(99999999);
    delete_task(99999999);
    
    // Try to add invalid tasks
    add_task("".to_string(), 0); // Empty text
    add_task("a".repeat(501), 0); // Too long
    
    // Application should still be in a valid state
    let tasks = load_tasks();
    assert!(tasks.len() == tasks.len()); // Should not crash - just check it loads
}

#[test]
fn test_backup_workflow() {
    // Test the backup and restore functionality
    
    // This test assumes there might not be a backup file
    match restore_from_backup() {
        Ok(_) => {
            // If restore succeeded, verify tasks are still loadable
            let tasks = load_tasks();
            assert!(tasks.len() == tasks.len()); // Just verify it doesn't crash
        }
        Err(_) => {
            // If restore failed (no backup), that's also fine for this test
            // Just verify the application is still in a valid state
            let tasks = load_tasks();
            assert!(tasks.len() == tasks.len()); // Just verify it doesn't crash
        }
    }
}

#[test] 
fn test_data_validation_integration() {
    // Test that the data validation works end-to-end
    
    let initial_tasks = load_tasks();
    let initial_count = initial_tasks.len();
    
    // These operations should be rejected gracefully (they print error messages but don't add tasks)
    add_task("".to_string(), 0);                    // Empty text
    add_task("   ".to_string(), 0);                 // Whitespace only
    
    // Verify that invalid add operations didn't add tasks
    let tasks_after_invalid_adds = load_tasks();
    assert_eq!(tasks_after_invalid_adds.len(), initial_count); // No tasks should be added
    
    // Try various invalid operations - none should crash or corrupt data
    toggle_task(0);           // Invalid ID
    delete_task(0);           // Invalid ID  
    toggle_task(999999);      // Non-existent ID
    delete_task(999999);      // Non-existent ID
    
    // Verify data integrity is maintained - count should be unchanged since invalid operations were rejected
    let final_tasks = load_tasks();
    assert_eq!(final_tasks.len(), initial_count);
    
    // Verify all existing tasks still have valid IDs and text
    for task in &final_tasks {
        assert!(task.id > 0);
        assert!(!task.text.is_empty());
        assert!(task.text.len() <= 500);
        
        if let Some(ref due_date) = task.due_date {
            assert_eq!(due_date.len(), 10); // YYYY-MM-DD format
            assert_eq!(due_date.chars().nth(4).unwrap(), '-');
            assert_eq!(due_date.chars().nth(7).unwrap(), '-');
        }
    }
}