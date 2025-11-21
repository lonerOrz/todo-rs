use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
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
    let mut path = dirs::config_dir().ok_or_else(|| anyhow::Error::msg("Failed to get config directory"))?;
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
    fs::write(path, json)?;
    Ok(())
}