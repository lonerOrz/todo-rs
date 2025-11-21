use crate::model::{load_tasks, save_tasks, Task};
use anyhow::Result;
use std::cell::RefCell;

// Using RefCell to implement interior mutability, simplifying singleton pattern
thread_local! {
    static TASK_STORE: RefCell<Option<Vec<Task>>> = RefCell::new(None);
}

pub struct TaskStore;

impl TaskStore {
    /// Initialize task storage, loading tasks from disk
    pub fn init() -> Result<()> {
        TASK_STORE.with(|store| {
            let tasks = load_tasks()?;
            *store.borrow_mut() = Some(tasks);
            Ok(())
        })
    }

    /// Get all tasks
    pub fn get_all_tasks() -> Result<Vec<Task>> {
        TASK_STORE.with(|store| {
            let borrowed = store.borrow();
            match borrowed.as_ref() {
                Some(tasks) => Ok(tasks.clone()),
                None => Err(anyhow::anyhow!("Task store not initialized")),
            }
        })
    }

    /// Add a new task
    pub fn add_task(task: Task) -> Result<()> {
        TASK_STORE.with(|store| {
            let mut borrowed = store.borrow_mut();
            if let Some(ref mut tasks) = *borrowed {
                tasks.push(task);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Task store not initialized"))
            }
        })
    }

    /// Update a task
    pub fn update_task(id: usize, updated_task: Task) -> Result<bool> {
        TASK_STORE.with(|store| {
            let mut borrowed = store.borrow_mut();
            if let Some(ref mut tasks) = *borrowed {
                if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                    tasks[pos] = updated_task;
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(anyhow::anyhow!("Task store not initialized"))
            }
        })
    }

    /// Remove a task
    pub fn remove_task(id: usize) -> Result<bool> {
        TASK_STORE.with(|store| {
            let mut borrowed = store.borrow_mut();
            if let Some(ref mut tasks) = *borrowed {
                if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                    tasks.remove(pos);
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(anyhow::anyhow!("Task store not initialized"))
            }
        })
    }

    /// Get maximum ID
    pub fn get_max_id() -> Result<usize> {
        TASK_STORE.with(|store| {
            let borrowed = store.borrow();
            if let Some(tasks) = borrowed.as_ref() {
                Ok(tasks.iter().map(|t| t.id).max().unwrap_or(0))
            } else {
                Err(anyhow::anyhow!("Task store not initialized"))
            }
        })
    }

    /// Find task by ID
    pub fn find_task_by_id(id: usize) -> Result<Option<Task>> {
        TASK_STORE.with(|store| {
            let borrowed = store.borrow();
            if let Some(tasks) = borrowed.as_ref() {
                Ok(tasks.iter().find(|t| t.id == id).cloned())
            } else {
                Err(anyhow::anyhow!("Task store not initialized"))
            }
        })
    }

    /// Save all tasks to disk
    pub fn save_to_disk() -> Result<()> {
        TASK_STORE.with(|store| {
            let borrowed = store.borrow();
            if let Some(tasks) = borrowed.as_ref() {
                save_tasks(tasks)?;
            } else {
                return Err(anyhow::anyhow!("Task store not initialized"));
            }
            Ok(())
        })
    }

    /// Filter tasks by condition
    pub fn filter_tasks<F>(predicate: F) -> Result<Vec<Task>>
    where
        F: Fn(&Task) -> bool,
    {
        TASK_STORE.with(|store| {
            let borrowed = store.borrow();
            if let Some(tasks) = borrowed.as_ref() {
                Ok(tasks.iter().filter(|&task| predicate(task)).cloned().collect())
            } else {
                Err(anyhow::anyhow!("Task store not initialized"))
            }
        })
    }
}