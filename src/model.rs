use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub task: String,
    pub date: String,
    pub done: bool,
    #[serde(default)]
    pub reuse_by: Option<usize>,
}

pub fn get_storage_path() -> Result<PathBuf> {
    // Check for a test-specific environment variable first
    if let Ok(test_path) = std::env::var("TD_TEST_CONFIG_DIR") {
        let mut path = PathBuf::from(test_path);
        path.push("td-rs/todo.json");
        return Ok(path);
    }

    let mut path =
        dirs::config_dir().ok_or_else(|| anyhow::Error::msg("Failed to get config directory"))?;
    path.push("td-rs/todo.json");
    Ok(path)
}

pub fn today_str() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn load_tasks() -> Result<Vec<Task>> {
    let path = get_storage_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let path = get_storage_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(tasks)?;
    let mut file = fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_get_storage_path() {
        let result = get_storage_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("td-rs/todo.json"));
    }

    #[test]
    fn test_today_str_format() {
        let today = today_str();
        // Verify the date format is YYYY-MM-DD
        assert!(today.len() == 10);
        assert!(today.chars().nth(4) == Some('-'));
        assert!(today.chars().nth(7) == Some('-'));
    }

    #[test]
    fn test_save_and_load_tasks() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".config");
        fs::create_dir_all(&config_dir).unwrap();
        let storage_path = config_dir.join("td-rs").join("todo.json");

        // Create a temporary task list
        let tasks = vec![
            Task {
                id: 1,
                task: "Test task".to_string(),
                date: "2023-01-01".to_string(),
                done: false,
                reuse_by: None,
            },
            Task {
                id: 2,
                task: "Another test task".to_string(),
                date: "2023-01-02".to_string(),
                done: true,
                reuse_by: Some(1),
            },
        ];

        // Save tasks with our helper function that uses the temp path
        let result = save_tasks_with_path(&tasks, &storage_path);
        assert!(result.is_ok());

        // Load tasks from the temp path
        let loaded_tasks = load_tasks_from_path(&storage_path).unwrap();
        assert_eq!(loaded_tasks.len(), 2);
        assert_eq!(loaded_tasks[0].id, 1);
        assert_eq!(loaded_tasks[0].task, "Test task");
        assert_eq!(loaded_tasks[0].date, "2023-01-01");
        assert_eq!(loaded_tasks[0].done, false);
        assert_eq!(loaded_tasks[0].reuse_by, None);
        assert_eq!(loaded_tasks[1].id, 2);
        assert_eq!(loaded_tasks[1].task, "Another test task");
        assert_eq!(loaded_tasks[1].date, "2023-01-02");
        assert_eq!(loaded_tasks[1].done, true);
        assert_eq!(loaded_tasks[1].reuse_by, Some(1));
    }

    // Helper function for testing, using a specified path instead of default
    fn save_tasks_with_path(tasks: &[Task], path: &PathBuf) -> Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let json = serde_json::to_string_pretty(tasks)?;
        let mut file = fs::File::create(&path)?;
        file.write_all(json.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    // Helper function for testing, using a specified path instead of default
    fn load_tasks_from_path(path: &PathBuf) -> Result<Vec<Task>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let data = fs::read_to_string(path)?;
        let tasks = serde_json::from_str(&data)?;
        Ok(tasks)
    }
}
