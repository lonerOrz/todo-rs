use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub task: String,
    pub date: String,
    pub done: bool,
}

pub fn get_storage_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("td-rs/todo.json");
    path
}

pub fn today_str() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn load_tasks() -> Vec<Task> {
    let path = get_storage_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_tasks(tasks: &[Task]) {
    let path = get_storage_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).ok();
    }
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(path, json).unwrap();
}
