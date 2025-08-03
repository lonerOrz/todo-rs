use crate::model::*;

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

pub fn list(date: Option<String>) {
    let tasks = load_tasks();
    let filter_date = date.unwrap_or_else(today_str);
    println!("Tasks for {}:", filter_date);
    for t in tasks.iter().filter(|t| t.date == filter_date) {
        let status = if t.done { "[✓]" } else { "[ ]" };
        println!("{} {} {}", t.id, status, t.task);
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

pub fn count_today() {
    let tasks = load_tasks();
    let today = today_str();
    let count = tasks.iter().filter(|t| !t.done && t.date == today).count();
    println!("{}", count);
}

pub fn prompt_today() {
    let tasks = load_tasks();
    let today = today_str();
    let parts: Vec<String> = tasks.iter()
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
