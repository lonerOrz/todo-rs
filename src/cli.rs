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

    let mut tasks_to_display: Vec<&Task> = Vec::new();

    if let Some(specific_date_str) = date_arg {
        // Specific date view
        let specific_date =
            parse_date_str(&specific_date_str).expect("Invalid date format. Use YYYY-MM-DD.");
        tasks_to_display = all_tasks
            .iter()
            .filter(|t| parse_date_str(&t.date).map_or(false, |d| d == specific_date))
            .collect();
        println!("--- For {} ---", specific_date_str);
    } else if show_week {
        // Week view
        let (week_start, week_end) = get_current_week_range(today_date);
        tasks_to_display = all_tasks
            .iter()
            .filter(|t| parse_date_str(&t.date).map_or(false, |d| d >= week_start && d <= week_end))
            .collect();
        println!(
            "--- For Current Week ({}) ---",
            today_date.format("%Y-%m-%d")
        );
    } else if show_month {
        // Month view
        let (month_start, month_end) = get_current_month_range(today_date);
        tasks_to_display = all_tasks
            .iter()
            .filter(|t| {
                parse_date_str(&t.date).map_or(false, |d| d >= month_start && d <= month_end)
            })
            .collect();
        println!("--- For Current Month ({}) ---", today_date.format("%Y-%m"));
    } else {
        // Default view: Today's tasks + overdue tasks (up to 7 days)
        let mut overdue_tasks: Vec<&Task> = all_tasks
            .iter()
            .filter(|t| {
                let task_date = parse_date_str(&t.date).expect("Invalid date format in task data.");
                !t.done && task_date < today_date && (today_date - task_date).num_days() <= 7
            })
            .collect();
        overdue_tasks.sort_by_key(|t| &t.date); // Sort overdue by date

        let mut todays_tasks: Vec<&Task> = all_tasks
            .iter()
            .filter(|t| parse_date_str(&t.date).map_or(false, |d| d == today_date))
            .collect();
        todays_tasks.sort_by_key(|t| t.id); // Sort today's by ID

        if !overdue_tasks.is_empty() {
            println!(
                "
--- Overdue Tasks (up to 7 days) ---"
            );
            for t in &overdue_tasks {
                let days_overdue = (today_date - parse_date_str(&t.date).unwrap()).num_days();
                println!("{} [ ] {} (过期 {} 天)", t.id, t.task, days_overdue);
            }
        }

        if !todays_tasks.is_empty() {
            println!(
                "
--- Today's Tasks ({}) ---",
                today
            );
            for t in &todays_tasks {
                let status = if t.done { "[✓]" } else { "[ ]" };
                println!("{} {} {}", t.id, status, t.task);
            }
        }

        if overdue_tasks.is_empty() && todays_tasks.is_empty() {
            println!("No tasks for today or recent overdue tasks.");
        }
        return; // Exit after default view
    }

    // For specific date, week, or month views, sort and display
    tasks_to_display.sort_by_key(|t| &t.date); // Sort by date for consistency
    if tasks_to_display.is_empty() {
        println!("No tasks found for this period.");
    } else {
        for t in tasks_to_display {
            let status = if t.done { "[✓]" } else { "[ ]" };
            let overdue_info =
                if !t.done && parse_date_str(&t.date).map_or(false, |d| d < today_date) {
                    let days_overdue = (today_date - parse_date_str(&t.date).unwrap()).num_days();
                    format!(" (过期 {} 天)", days_overdue)
                } else {
                    "".to_string()
                };
            println!("{} {} {}{}", t.id, status, t.task, overdue_info);
        }
    }
}

// Helper function to parse date strings
fn parse_date_str(date_str: &str) -> Result<chrono::NaiveDate, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

// Helper function to get current week range (Monday to Sunday)
fn get_current_week_range(today: chrono::NaiveDate) -> (chrono::NaiveDate, chrono::NaiveDate) {
    let weekday = today.weekday();
    let monday = today - chrono::Duration::days(weekday.num_days_from_monday() as i64);
    let sunday = monday + chrono::Duration::days(6);
    (monday, sunday)
}

// Helper function to get current month range
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

// Helper function to safely truncate string by characters
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
    let tasks = load_tasks();

    let today = today_str();

    let parts: Vec<String> = tasks
        .iter()
        .filter(|t| t.date == today)
        .map(|t| {
            let icon = if t.done { "🟢" } else { "🔴" };

            format!("{}#{}", icon, t.id)
        })
        .collect();

    if !parts.is_empty() {
        println!("{}", parts.join(" "));
    }
}

pub fn review() {
    let all_tasks = load_tasks();
    let today_date = parse_date_str(&today_str()).expect("Failed to parse today's date");

    println!("\n--- Tasks Overdue by More Than 7 Days ---");

    let mut old_overdue_tasks: Vec<&Task> = all_tasks
        .iter()
        .filter(|t| {
            let task_date = parse_date_str(&t.date).expect("Invalid date format in task data.");
            !t.done && task_date < today_date && (today_date - task_date).num_days() > 7
        })
        .collect();

    old_overdue_tasks.sort_by_key(|t| &t.date);

    if old_overdue_tasks.is_empty() {
        println!("No tasks currently overdue by more than 7 days.");
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
        println!("- 重新安排日期：td add \"任务描述\" --date YYYY-MM-DD (然后 td rm <旧任务ID>)");
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
