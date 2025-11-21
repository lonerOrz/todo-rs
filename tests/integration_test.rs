use std::process::Command;
use tempfile::TempDir;

// Helper function: Set up test environment
fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Set environment variable to make td use temporary directory for storage
    std::env::set_var("HOME", temp_dir.path());

    temp_dir
}

#[test]
fn test_full_workflow() {
    let _temp_dir = setup_test_env();

    // Test adding a task
    let output = Command::new("cargo")
        .args(&["run", "--", "add", "Test task 1"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Added task"));

    // Test listing tasks
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Test task 1"));

    // Test marking task as done
    let output = Command::new("cargo")
        .args(&["run", "--", "done", "1"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("marked done"));

    // List tasks again to verify status has been updated
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("[✓]")); // Should show as done

    // Test removing a task
    let output = Command::new("cargo")
        .args(&["run", "--", "rm", "1"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("removed"));

    // Verify the task has been removed
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains("Test task 1"));
}

#[test]
fn test_add_with_date() {
    let _temp_dir = setup_test_env();

    // Test adding a task with a date
    let output = Command::new("cargo")
        .args(&["run", "--", "add", "Test task with date", "--date", "2023-12-25"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Added task"));
}

#[test]
fn test_error_handling() {
    let _temp_dir = setup_test_env();

    // Test trying to mark a non-existent task as done
    let output = Command::new("cargo")
        .args(&["run", "--", "done", "999"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success()); // Program should handle error gracefully
    // The error message might be different, so just check that it doesn't crash
    // and that the execution was successful
}