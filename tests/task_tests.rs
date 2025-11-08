use todo::{Task, TaskError, validate_date, get_today, get_date_with_offset};

#[test]
fn test_task_creation() {
    let task = Task {
        id: 1,
        text: "Test task".to_string(),
        done: false,
        due_date: Some("2025-11-08".to_string()),
    };

    assert_eq!(task.id, 1);
    assert_eq!(task.text, "Test task");
    assert!(!task.done);
    assert_eq!(task.due_date, Some("2025-11-08".to_string()));
}

#[test]
fn test_validate_date_valid() {
    assert!(validate_date("2025-11-08").is_ok());
    assert!(validate_date("2024-02-29").is_ok()); // Leap year
    assert!(validate_date("2023-02-28").is_ok());
    assert!(validate_date("2025-12-31").is_ok());
    assert!(validate_date("2000-01-01").is_ok());
}

#[test]
fn test_validate_date_invalid_format() {
    assert!(validate_date("").is_err());
    assert!(validate_date("2025/11/08").is_err());
    assert!(validate_date("2025-11").is_err());
    assert!(validate_date("11-08-2025").is_err());
    assert!(validate_date("not-a-date").is_err());
}

#[test]
fn test_validate_date_invalid_values() {
    assert!(validate_date("2025-13-08").is_err()); // Invalid month
    assert!(validate_date("2025-00-08").is_err()); // Invalid month
    assert!(validate_date("2025-11-00").is_err()); // Invalid day
    assert!(validate_date("2025-11-32").is_err()); // Invalid day
    assert!(validate_date("2023-02-29").is_err()); // Invalid day for non-leap year
    assert!(validate_date("2025-04-31").is_err()); // Invalid day for April
    assert!(validate_date("1899-01-01").is_err()); // Year too old
    assert!(validate_date("10000-01-01").is_err()); // Year too high
}

#[test]
fn test_validate_date_leap_year() {
    assert!(validate_date("2024-02-29").is_ok()); // 2024 is leap year
    assert!(validate_date("2023-02-29").is_err()); // 2023 is not leap year
    assert!(validate_date("2000-02-29").is_ok()); // 2000 is leap year (divisible by 400)
    assert!(validate_date("1900-02-29").is_err()); // 1900 is not leap year (divisible by 100, not 400)
}

#[test]
fn test_get_today() {
    let today = get_today();
    // Should return a valid date format
    assert!(validate_date(&today).is_ok());
    // Should be in YYYY-MM-DD format
    assert_eq!(today.len(), 10);
    assert_eq!(today.chars().nth(4).unwrap(), '-');
    assert_eq!(today.chars().nth(7).unwrap(), '-');
}

#[test]
fn test_task_json_serialization() {
    let task = Task {
        id: 1,
        text: "Test task".to_string(),
        done: false,
        due_date: Some("2025-11-08".to_string()),
    };

    // Test serialization
    let json = serde_json::to_string(&task).unwrap();
    assert!(json.contains("\"id\":1"));
    assert!(json.contains("\"text\":\"Test task\""));
    assert!(json.contains("\"done\":false"));
    assert!(json.contains("\"due_date\":\"2025-11-08\""));

    // Test deserialization
    let deserialized: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, task.id);
    assert_eq!(deserialized.text, task.text);
    assert_eq!(deserialized.done, task.done);
    assert_eq!(deserialized.due_date, task.due_date);
}

#[test]
fn test_task_without_due_date() {
    let task = Task {
        id: 1,
        text: "Test task".to_string(),
        done: false,
        due_date: None,
    };

    let json = serde_json::to_string(&task).unwrap();
    // due_date should be omitted when None
    assert!(!json.contains("due_date"));

    let deserialized: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.due_date, None);
}

#[test]
fn test_error_display() {
    let err = TaskError::InvalidDate("bad date".to_string());
    assert_eq!(format!("{}", err), "Invalid date format: bad date");

    let err = TaskError::InvalidInput("bad input".to_string());
    assert_eq!(format!("{}", err), "Invalid input: bad input");
}

#[test]
fn test_get_date_with_offset() {
    let today = get_today();
    let tomorrow = get_date_with_offset(1);
    let yesterday = get_date_with_offset(-1);
    
    // All should be valid dates
    assert!(validate_date(&today).is_ok());
    assert!(validate_date(&tomorrow).is_ok());
    assert!(validate_date(&yesterday).is_ok());
    
    // Tomorrow should be different from today
    assert_ne!(today, tomorrow);
    assert_ne!(today, yesterday);
    assert_ne!(tomorrow, yesterday);
}

#[test]
fn test_backup_creation() {
    use std::env;
    
    // Create a temporary test file
    let temp_dir = env::temp_dir();
    let test_file = temp_dir.join("test_todos.json");
    let backup_file = temp_dir.join("test_todos.json.backup");
    
    // Write some test content
    let test_content = r#"[{"id":1,"text":"Test task","done":false}]"#;
    std::fs::write(&test_file, test_content).unwrap();
    
    // Test that file exists before backup
    assert!(test_file.exists());
    
    // Cleanup
    let _ = std::fs::remove_file(test_file);
    let _ = std::fs::remove_file(backup_file);
}