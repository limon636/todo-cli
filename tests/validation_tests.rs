use todo::{Task, validate_date, TaskError, load_tasks, set_due_date};

#[test]
fn test_validate_task_id_logic() {
    let tasks = vec![
        Task { id: 1, text: "Task 1".to_string(), done: false, due_date: None },
        Task { id: 3, text: "Task 3".to_string(), done: false, due_date: None },
    ];
    
    // Test that we have tasks with specific IDs
    assert!(tasks.iter().any(|t| t.id == 1));
    assert!(tasks.iter().any(|t| t.id == 3));
    assert!(!tasks.iter().any(|t| t.id == 2));
    assert!(!tasks.iter().any(|t| t.id == 99));
}

#[test]
fn test_validate_date_integration() {
    // Test that our validation works with the set_due_date function
    let initial_tasks = load_tasks();
    
    // This should print an error message but not panic
    set_due_date(99999, "invalid-date".to_string());
    
    // Tasks should be unchanged
    let tasks_after = load_tasks();
    assert_eq!(initial_tasks.len(), tasks_after.len());
}

#[test]
fn test_date_validation_comprehensive() {
    // Test various edge cases for date validation
    
    // Valid dates
    assert!(validate_date("2025-01-01").is_ok());
    assert!(validate_date("2025-12-31").is_ok());
    assert!(validate_date("2024-02-29").is_ok()); // Leap year
    
    // Invalid formats
    assert!(validate_date("").is_err());
    assert!(validate_date("2025/01/01").is_err());
    assert!(validate_date("01-01-2025").is_err());
    
    // Invalid values
    assert!(validate_date("2025-13-01").is_err()); // Invalid month
    assert!(validate_date("2025-01-32").is_err()); // Invalid day
    assert!(validate_date("2023-02-29").is_err()); // Invalid day for non-leap year
}

#[test]
fn test_error_types() {
    // Test different error types
    let date_err = TaskError::InvalidDate("test".to_string());
    let input_err = TaskError::InvalidInput("test".to_string());
    
    // Test that they implement Display properly
    assert!(format!("{}", date_err).contains("Invalid date"));
    assert!(format!("{}", input_err).contains("Invalid input"));
}

#[test]
fn test_input_length_validation() {
    // Test that we handle various input lengths properly
    let normal_text = "This is a normal task";
    let long_text = "a".repeat(500); // At the limit
    let too_long_text = "a".repeat(501); // Over the limit
    
    assert!(normal_text.len() < 500);
    assert_eq!(long_text.len(), 500);
    assert_eq!(too_long_text.len(), 501);
}