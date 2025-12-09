use assert_cmd::Command;
use chrono::{Duration, Local};
use predicates::prelude::*;
use tempfile::TempDir;

// Test structure that manages the temporary directory for each test
struct TestEnv {
    temp_dir: TempDir,
}

impl TestEnv {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");

        // Set environment variable to make td use temporary directory for storage
        std::env::set_var("XDG_CONFIG_HOME", temp_dir.path().to_str().unwrap());

        TestEnv { temp_dir }
    }

    fn td_command(&self) -> Command {
        let mut cmd = Command::cargo_bin("td").expect("Failed to find td binary");
        cmd.env("XDG_CONFIG_HOME", self.temp_dir.path().to_str().unwrap());
        cmd
    }
}

#[test]
fn test_full_workflow() {
    let env = TestEnv::new();

    // Test adding a task
    env.td_command()
        .args(&["add", "Test task 1"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Added task #"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Test listing tasks
    env.td_command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("Test task 1"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Test marking task as done
    env.td_command()
        .args(&["done", "1"])
        .assert()
        .success()
        .stdout(predicates::str::contains("marked done."));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // List tasks again to verify status has been updated
    env.td_command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("[✓] Test task 1")); // Should show as done
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Test removing a task
    env.td_command()
        .args(&["rm", "1"])
        .assert()
        .success()
        .stdout(predicates::str::contains("removed."));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify the task has been removed
    env.td_command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "No tasks for today or overdue this week.",
        )); // Should be empty
}

#[test]
fn test_add_with_date() {
    let env = TestEnv::new();

    // Test adding a task with a date
    env.td_command()
        .args(&[
            "add",
            "Test task with specific date",
            "--date",
            "2023-12-25",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Added task #"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify the task was added with correct date by listing for that date
    env.td_command()
        .args(&["list", "--date", "2023-12-25"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Test task with specific date"));
}

#[test]
fn test_edit_task() {
    let env = TestEnv::new();

    // Add a task first
    env.td_command()
        .args(&["add", "Original task description"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Added task #"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Edit the task description
    env.td_command()
        .args(&["edit", "1", "--task", "Updated task description"])
        .assert()
        .success()
        .stdout(predicates::str::contains("updated."));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify the task was updated
    env.td_command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("Updated task description"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Edit the task date
    env.td_command()
        .args(&["edit", "1", "--date", "2024-03-15"])
        .assert()
        .success()
        .stdout(predicates::str::contains("updated."));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify the date was updated
    env.td_command()
        .args(&["list", "--date", "2024-03-15"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Updated task description"));
}

#[test]
fn test_error_handling_non_existent_task() {
    let env = TestEnv::new();

    // Test trying to mark a non-existent task as done
    env.td_command()
        .args(&["done", "999"])
        .assert()
        .success() // td exits with 0 even on task not found
        .stderr(predicates::str::contains("Task #999 not found."));

    // Test trying to remove a non-existent task
    env.td_command()
        .args(&["rm", "999"])
        .assert()
        .success() // td exits with 0 even on task not found
        .stderr(predicates::str::contains("Task #999 not found."));

    // Test trying to edit a non-existent task
    env.td_command()
        .args(&["edit", "999", "--task", "Non existent"])
        .assert()
        .success() // td exits with 0 even on task not found
        .stderr(predicates::str::contains("Task #999 not found."));
}

#[test]
fn test_review_command() {
    let env = TestEnv::new();
    let today = Local::now().date_naive();
    let _seven_days_ago = today - Duration::days(7);
    let nine_days_ago = today - Duration::days(9);

    // Add an overdue task (9 days ago)
    env.td_command()
        .args(&[
            "add",
            "Very overdue task",
            "--date",
            &nine_days_ago.to_string(),
        ])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Add a task that's not overdue enough for review (6 days ago)
    env.td_command()
        .args(&[
            "add",
            "Not overdue enough task",
            "--date",
            &(today - Duration::days(6)).to_string(),
        ])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Add a current task
    env.td_command()
        .args(&["add", "Current task", "--date", &today.to_string()])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Run the review command
    env.td_command()
        .arg("review")
        .assert()
        .success()
        .stdout(predicates::str::contains("Very overdue task")) // Should list this one
        .stdout(predicates::str::contains("9")) // Should show correct overdue days
        .stdout(predicates::str::contains(&nine_days_ago.to_string()))
        .stdout(predicates::str::contains("Not overdue enough task").not()) // Should NOT list this one
        .stdout(predicates::str::contains("Current task").not()); // Should NOT list this one
}

#[test]
fn test_reuse_command() {
    let env = TestEnv::new();
    let tomorrow = (Local::now().date_naive() + Duration::days(1)).to_string();

    // Add an original task
    env.td_command()
        .args(&["add", "Original task for reuse"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Added task #"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Capture the task ID from stdout or just continue with ID 1
    // Reuse task #1 with a new date
    env.td_command()
        .args(&["reuse", "1", "--date", &tomorrow])
        .assert()
        .success()
        .stdout(
            predicates::str::contains("as new task #")
                .and(predicates::str::contains("Original task marked done.")),
        );
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify original task #1 is marked done
    env.td_command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("[✓] Original task for reuse"));
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify new task exists and is not done, for tomorrow, and references #1
    env.td_command()
        .args(&["list", "--date", &tomorrow])
        .assert()
        .success()
        .stdout(predicates::str::contains("Original task for reuse"))
        .stdout(predicates::str::contains("reused from #1"));
}

#[test]
fn test_list_commands() {
    let env = TestEnv::new();
    let today = Local::now().date_naive();
    let tomorrow = today + Duration::days(1);
    let yesterday = today - Duration::days(1);
    let next_week = today + Duration::days(8);
    let last_week = today - Duration::days(8);

    // Add various tasks
    env.td_command()
        .args(&["add", "Task today"])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));
    env.td_command()
        .args(&["add", "Task tomorrow", "--date", &tomorrow.to_string()])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));
    env.td_command()
        .args(&["add", "Task yesterday", "--date", &yesterday.to_string()])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));
    env.td_command()
        .args(&["add", "Task next week", "--date", &next_week.to_string()])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));
    env.td_command()
        .args(&["add", "Task last week", "--date", &last_week.to_string()])
        .assert()
        .success();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // List default (today and overdue this week)
    env.td_command()
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("Task today"))
        .stdout(predicates::str::contains("Task yesterday")) // Overdue from this week
        .stdout(predicates::str::contains("Task tomorrow").not())
        .stdout(predicates::str::contains("Task next week").not())
        .stdout(predicates::str::contains("Task last week").not());

    // List by specific date (tomorrow)
    env.td_command()
        .args(&["list", "--date", &tomorrow.to_string()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Task tomorrow"))
        .stdout(predicates::str::contains("Task today").not())
        .stdout(predicates::str::contains("Task yesterday").not());

    // List by week (should include today, tomorrow, yesterday, but not next_week, last_week if they are outside current week)
    // This test is a bit sensitive to the current date and week boundaries.
    // For simplicity, we'll check for tasks expected within a typical week range (Mon-Sun).
    env.td_command()
        .args(&["list", "--week"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Task today"))
        .stdout(predicates::str::contains("Task tomorrow"))
        .stdout(predicates::str::contains("Task yesterday"));
    // Depending on `today`, next_week and last_week might or might not be in the current week.
    // We'll avoid asserting their absence for now to keep the test simple and robust against date changes.

    // List by month
    env.td_command()
        .args(&["list", "--month"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Task today"))
        .stdout(predicates::str::contains("Task tomorrow"))
        .stdout(predicates::str::contains("Task yesterday"))
        .stdout(predicates::str::contains("Task next week"))
        .stdout(predicates::str::contains("Task last week")); // Should contain all tasks in the current month
}

#[test]
fn test_prompt_today_command() {
    let env = TestEnv::new();
    let today = Local::now().date_naive();
    let yesterday = today - Duration::days(1);
    let two_days_ago = today - Duration::days(2);

    // Add tasks

    env.td_command()
        .args(&["add", "Task for today 1"])
        .assert()
        .success(); // ID 1

    std::thread::sleep(std::time::Duration::from_millis(100));

    env.td_command()
        .args(&["add", "Task for today 2"])
        .assert()
        .success(); // ID 2

    std::thread::sleep(std::time::Duration::from_millis(100));

    env.td_command()
        .args(&[
            "add",
            "Task for yesterday",
            "--date",
            &yesterday.to_string(),
        ])
        .assert()
        .success(); // ID 3

    std::thread::sleep(std::time::Duration::from_millis(100));

    env.td_command()
        .args(&[
            "add",
            "Done task yesterday",
            "--date",
            &two_days_ago.to_string(),
        ])
        .assert()
        .success(); // ID 4

    std::thread::sleep(std::time::Duration::from_millis(100));

    env.td_command().args(&["done", "4"]).assert().success();

    std::thread::sleep(std::time::Duration::from_millis(100));

    // Check prompt-today output
    env.td_command()
        .arg("prompt-today")
        .assert()
        .success()
        .stdout(predicates::str::contains("🔴#1")) // Undone today
        .stdout(predicates::str::contains("🔴#2")) // Undone today
        .stdout(predicates::str::contains("🔴#3")); // Undone yesterday (overdue this week)
                                                    // Note: #4 is done, so it might not appear in prompt-today output
}
