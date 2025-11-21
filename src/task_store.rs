use crate::model::{load_tasks, save_tasks, Task};
use anyhow::Result;
use std::cell::RefCell;

thread_local! {
    static TASK_STORE: RefCell<Option<Vec<Task>>> = RefCell::new(None);
}

pub struct TaskStore;

impl TaskStore {
    pub fn init() -> Result<()> {
        TASK_STORE.with(|store| {
            let tasks = match load_tasks() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Warning: Could not load tasks ({}). Initializing with empty store.", e);
                    // Optionally, try to remove the corrupted file if it exists and is the cause
                    if let Ok(path) = crate::model::get_storage_path() {
                        if path.exists() {
                            eprintln!("Attempting to delete corrupted task file: {:?}", path);
                            if let Err(remove_err) = std::fs::remove_file(path) {
                                eprintln!("Error deleting corrupted file: {}", remove_err);
                            }
                        }
                    }
                    Vec::new()
                }
            };
            *store.borrow_mut() = Some(tasks);
            Ok(())
        })
    }

    pub fn get_all_tasks() -> Result<Vec<Task>> {
        TASK_STORE.with(|store| {
            let borrowed = store.borrow();
            match borrowed.as_ref() {
                Some(tasks) => Ok(tasks.clone()),
                None => Err(anyhow::anyhow!("Task store not initialized")),
            }
        })
    }

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

    #[cfg(test)]
    pub fn reset_store_for_testing() {
        TASK_STORE.with(|store| {
            *store.borrow_mut() = Some(vec![]);
        });
    }

    #[allow(dead_code)] // Allow dead code in non-test builds
    pub fn reset_for_testing_integration() {
        #[cfg(test)]
        TASK_STORE.with(|store| {
            *store.borrow_mut() = Some(vec![]);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task_store_init() {
        // This test needs to be run in isolation because of the thread_local storage
        // Reset the store for this test
        TASK_STORE.with(|store| {
            *store.borrow_mut() = Some(vec![]);
        });

        let result = TaskStore::init();
        assert!(result.is_ok());
    }

    #[test]
    fn test_task_store_operations() {
        // Initialize an empty store for testing
        TASK_STORE.with(|store| {
            *store.borrow_mut() = Some(vec![]);
        });

        // Test adding a task
        let task = Task {
            id: 1,
            task: "Test task".to_string(),
            date: "2023-01-01".to_string(),
            done: false,
            reuse_by: None,
        };

        let result = TaskStore::add_task(task.clone());
        assert!(result.is_ok());

        // Test getting max ID
        let max_id = TaskStore::get_max_id().unwrap();
        assert_eq!(max_id, 1);

        // Test finding the task by ID
        let found_task = TaskStore::find_task_by_id(1).unwrap();
        assert!(found_task.is_some());
        assert_eq!(found_task.unwrap().task, "Test task");

        // Test updating the task
        let updated_task = Task {
            id: 1,
            task: "Updated task".to_string(),
            date: "2023-01-01".to_string(),
            done: true,
            reuse_by: None,
        };

        let update_result = TaskStore::update_task(1, updated_task);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), true);

        // Verify the update
        let updated_found_task = TaskStore::find_task_by_id(1).unwrap();
        assert!(updated_found_task.is_some());
        let task = updated_found_task.unwrap();
        assert_eq!(task.task, "Updated task");
        assert_eq!(task.done, true);

        // Test removing a task
        let remove_result = TaskStore::remove_task(1);
        assert!(remove_result.is_ok());
        assert_eq!(remove_result.unwrap(), true);

        // Verify the removal
        let removed_found_task = TaskStore::find_task_by_id(1).unwrap();
        assert!(removed_found_task.is_none());
    }

    #[test]
    fn test_task_not_found() {
        // Initialize an empty store for testing
        TASK_STORE.with(|store| {
            *store.borrow_mut() = Some(vec![]);
        });

        // Try to find a non-existent task
        let found_task = TaskStore::find_task_by_id(999).unwrap();
        assert!(found_task.is_none());

        // Try to update a non-existent task
        let updated_task = Task {
            id: 999,
            task: "Non-existent task".to_string(),
            date: "2023-01-01".to_string(),
            done: false,
            reuse_by: None,
        };
        let update_result = TaskStore::update_task(999, updated_task);
        assert!(update_result.is_ok());
        assert_eq!(update_result.unwrap(), false); // Should return false for non-existent task

        // Try to remove a non-existent task
        let remove_result = TaskStore::remove_task(999);
        assert!(remove_result.is_ok());
        assert_eq!(remove_result.unwrap(), false); // Should return false for non-existent task
    }
}