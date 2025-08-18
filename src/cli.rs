use crate::model::*;
use chrono::Datelike;

pub fn add(task: String, date: Option<String>) {
    let mut tasks = load_tasks();
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let task = Task {
        id: new_id,
        task,
        date: date.unwrap_or_else(today_str),
        done: false,
        reuse_by: None,
    };
    tasks.push(task);
    save_tasks(&tasks);
    println!("[+] Added task #{}", new_id);
}

pub fn list(date_arg: Option<String>, show_week: bool, show_month: bool) {
    let all_tasks = load_tasks();
    let today = today_str();
    let today_date = parse_date_str(&today).expect("Failed to parse today's date");

    println!("Tasks:");

    if let Some(specific_date_str) = date_arg {
        // Specific date view
        let specific_date =
            parse_date_str(&specific_date_str).expect("Invalid date format. Use YYYY-MM-DD.");
        let tasks_to_display: Vec<&Task> = all_tasks
            .iter()
            .filter(|t| parse_date_str(&t.date).map_or(false, |d| d == specific_date))
            .collect();
        println!("--- For {} ---", specific_date_str);
        for t in tasks_to_display {
            let status = if t.done { "[✓]" } else { "[ ]" };
            let mut extra_info_parts = Vec::new();
            if let Some(reuse_id) = t.reuse_by {
                extra_info_parts.push(format!("reused from #{}", reuse_id));
            }
            if !t.done {
                if let Ok(task_date) = parse_date_str(&t.date) {
                    if task_date < today_date {
                        let days_overdue = (today_date - task_date).num_days();
                        if days_overdue > 0 {
                            extra_info_parts.push(format!("overdue {} days", days_overdue));
                        }
                    }
                }
            }
            let extra_info = if extra_info_parts.is_empty() {
                "".to_string()
            } else {
                format!(" ({})", extra_info_parts.join(", "))
            };
            println!("{} {} {}{}", t.id, status, t.task, extra_info);
        }
    } else if show_week {
        // Week view
        let (week_start, week_end) = get_current_week_range(today_date);
        let tasks_to_display: Vec<&Task> = all_tasks
            .iter()
            .filter(|t| {
                if let Ok(task_date) = parse_date_str(&t.date) {
                    task_date >= week_start && task_date <= week_end
                } else {
                    false
                }
            })
            .collect();
        println!(
            "--- For Current Week ({} to {})",
            week_start.format("%Y-%m-%d"),
            week_end.format("%Y-%m-%d")
        );
        for t in tasks_to_display {
            let status = if t.done { "[✓]" } else { "[ ]" };
            let mut extra_info_parts = Vec::new();
            if let Some(reuse_id) = t.reuse_by {
                extra_info_parts.push(format!("reused from #{}", reuse_id));
            }
            if !t.done {
                if let Ok(task_date) = parse_date_str(&t.date) {
                    if task_date < today_date {
                        let days_overdue = (today_date - task_date).num_days();
                        if days_overdue > 0 {
                            extra_info_parts.push(format!("overdue {} days", days_overdue));
                        }
                    }
                }
            }
            let extra_info = if extra_info_parts.is_empty() {
                "".to_string()
            } else {
                format!(" ({})", extra_info_parts.join(", "))
            };
            println!("{} {} {}{}", t.id, status, t.task, extra_info);
        }
    } else if show_month {
        // Month view
        let (month_start, month_end) = get_current_month_range(today_date);
        let tasks_to_display: Vec<&Task> = all_tasks
            .iter()
            .filter(|t| {
                parse_date_str(&t.date).map_or(false, |d| d >= month_start && d <= month_end)
            })
            .collect();
        println!("--- For Current Month ({})", today_date.format("%Y-%m"));
        for t in tasks_to_display {
            let status = if t.done { "[✓]" } else { "[ ]" };
            let mut extra_info_parts = Vec::new();
            if let Some(reuse_id) = t.reuse_by {
                extra_info_parts.push(format!("reused from #{}", reuse_id));
            }
            if !t.done {
                if let Ok(task_date) = parse_date_str(&t.date) {
                    if task_date < today_date {
                        let days_overdue = (today_date - task_date).num_days();
                        if days_overdue > 0 {
                            extra_info_parts.push(format!("overdue {} days", days_overdue));
                        }
                    }
                }
            }
            let extra_info = if extra_info_parts.is_empty() {
                "".to_string()
            } else {
                format!(" ({})", extra_info_parts.join(", "))
            };
            println!("{} {} {}{}", t.id, status, t.task, extra_info);
        }
    } else {
        // Default view: Show tasks for the current week
        let (week_start, week_end) = get_current_week_range(today_date);
        let mut week_tasks: Vec<&Task> = all_tasks
            .iter()
            .filter(|t| {
                if let Ok(task_date) = parse_date_str(&t.date) {
                    task_date >= week_start && task_date <= week_end
                } else {
                    false
                }
            })
            .collect();
        week_tasks.sort_by_key(|t| &t.date);

        println!(
            "--- Tasks For Current Week ({} to {})",
            week_start.format("%Y-%m-%d"),
            week_end.format("%Y-%m-%d")
        );

        if week_tasks.is_empty() {
            println!("No tasks for the current week.");
        } else {
            for t in week_tasks {
                let status = if t.done { "[✓]" } else { "[ ]" };
                let mut extra_info_parts = Vec::new();
                if let Some(reuse_id) = t.reuse_by {
                    extra_info_parts.push(format!("reused from #{}", reuse_id));
                }
                if !t.done {
                    if let Ok(task_date) = parse_date_str(&t.date) {
                        if task_date < today_date {
                            let days_overdue = (today_date - task_date).num_days();
                            if days_overdue > 0 {
                                extra_info_parts.push(format!("overdue {} days", days_overdue));
                            }
                        }
                    }
                }
                let extra_info = if extra_info_parts.is_empty() {
                    "".to_string()
                } else {
                    format!(" ({})", extra_info_parts.join(", "))
                };
                println!("{:<10}  {} {} {}{}", t.date, t.id, status, t.task, extra_info);
            }
        }
    }
}

pub fn mark_done(id: usize) {
    let mut tasks = load_tasks();
    for t in &mut tasks {
        if t.id == id {
            t.done = true;
            save_tasks(&tasks);
            println!("[✓] Task #{} marked done.", id);
            return;
        }
    }
    eprintln!("Task #{} not found.", id);
}

pub fn remove(id: usize) {
    let mut tasks = load_tasks();
    let before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() < before {
        save_tasks(&tasks);
        println!("[-] Task #{} removed.", id);
    } else {
        eprintln!("Task #{} not found.", id);
    }
}

pub fn prompt_today() {
    let all_tasks = load_tasks();
    let today = today_str();
    let today_date = parse_date_str(&today).expect("Failed to parse today's date");

    let mut parts: Vec<String> = Vec::new();

    let (week_start, week_end) = get_current_week_range(today_date);
    let week_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| {
            if let Ok(task_date) = parse_date_str(&t.date) {
                task_date >= week_start && task_date <= week_end
            } else {
                false
            }
        })
        .collect();

    for t in week_tasks {
        let icon = if t.done {
            "🟢"
        } else if t.reuse_by.is_some() {
            "🟡"
        } else {
            "🔴"
        };
        parts.push(format!("{}#{}", icon, t.id));
    }

    if !parts.is_empty() {
        println!("{}", parts.join(" "));
    }
}

pub fn review() {
    let all_tasks = load_tasks();
    let today_date = parse_date_str(&today_str()).expect("Failed to parse today's date");
    let (week_start, _) = get_current_week_range(today_date);

    println!("\n--- Tasks Overdue From Before Current Week ---");

    let mut old_overdue_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| {
            if let Ok(task_date) = parse_date_str(&t.date) {
                !t.done && task_date < week_start
            } else {
                false
            }
        })
        .collect();

    old_overdue_tasks.sort_by_key(|t| &t.date);

    if old_overdue_tasks.is_empty() {
        println!("No tasks currently overdue from before the current week.");
    } else {
        println!("ID   日期         过期天数   任务描述");
        println!("---  -----------  --------   -----------------------");
        for t in old_overdue_tasks {
            let days_overdue = (today_date - parse_date_str(&t.date).unwrap()).num_days();
            let task_display = truncate_string(&t.task, 22);
            println!(
                "{:<4} {:<11}  {:<4}     {}",
                t.id, t.date, days_overdue, task_display
            );
        }

        println!("\n您可以选择以下操作：");
        println!("- 完成任务：td done <任务ID>");
        println!("- 重新安排日期：td reuse <旧任务ID> --date YYYY-MM-DD");
        println!("- 删除任务：td rm <任务ID>");
    }
}

pub fn reuse(id: usize, date: Option<String>) {
    let mut tasks = load_tasks();
    if let Some(original_task) = tasks.iter().find(|t| t.id == id) {
        let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let new_task = Task {
            id: new_id,
            task: original_task.task.clone(),
            date: date.unwrap_or_else(today_str),
            done: false,
            reuse_by: Some(id),
        };
        tasks.push(new_task);
        // Mark the original task as done
        if let Some(original_task_mut) = tasks.iter_mut().find(|t| t.id == id) {
            original_task_mut.done = true;
        }
        save_tasks(&tasks);
        println!(
            "[+] Reused task #{} as new task #{}. Original task marked done.",
            id, new_id
        );
    } else {
        eprintln!("Task #{} not found.", id);
    }
}

// Helper functions that were originally in the file
fn parse_date_str(date_str: &str) -> Result<chrono::NaiveDate, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

fn get_current_week_range(today: chrono::NaiveDate) -> (chrono::NaiveDate, chrono::NaiveDate) {
    let weekday = today.weekday();
    let monday = today - chrono::Duration::days(weekday.num_days_from_monday() as i64);
    let sunday = monday + chrono::Duration::days(6);
    (monday, sunday)
}

fn get_current_month_range(today: chrono::NaiveDate) -> (chrono::NaiveDate, chrono::NaiveDate) {
    let year = today.year();
    let month = today.month();
    let month_start = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let month_end = if month == 12 {
        chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - chrono::Duration::days(1)
    } else {
        chrono::NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - chrono::Duration::days(1)
    };
    (month_start, month_end)
}

fn truncate_string(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        let mut truncated = s.chars().take(max_chars).collect::<String>();
        truncated.push_str("...");
        truncated
    } else {
        let num_spaces = max_chars - s.chars().count();
        let padding = " ".repeat(num_spaces);
        format!("{}{}", s, padding)
    }
}