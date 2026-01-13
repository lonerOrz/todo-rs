use clap::{Arg, Command};

mod cli;
mod model;
mod shell;
mod task_store;

fn main() {
    if let Err(e) = task_store::TaskStore::init() {
        eprintln!("Error initializing task store: {}", e);
        std::process::exit(1);
    }

    let matches = Command::new("td")
        .version("0.1.0")
        .about("Minimalistic CLI Todo")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("task").required(true))
                .arg(Arg::new("date").long("date").value_name("DATE")),
        )
        .subcommand(
            Command::new("list")
                .about("List tasks")
                .arg(
                    Arg::new("date")
                        .long("date")
                        .value_name("DATE")
                        .help("List tasks for a specific date (YYYY-MM-DD)"),
                )
                .arg(
                    Arg::new("week")
                        .long("week")
                        .action(clap::ArgAction::SetTrue)
                        .help("List tasks for the current week"),
                )
                .arg(
                    Arg::new("month")
                        .long("month")
                        .action(clap::ArgAction::SetTrue)
                        .help("List tasks for the current month"),
                )
                .arg(
                    Arg::new("done")
                        .long("done")
                        .action(clap::ArgAction::SetTrue)
                        .help("List only completed tasks"),
                )
                .arg(
                    Arg::new("pending")
                        .long("pending")
                        .action(clap::ArgAction::SetTrue)
                        .help("List only pending tasks"),
                )
                .arg(
                    Arg::new("from_id")
                        .long("from")
                        .value_name("ID")
                        .help("List tasks with ID greater than or equal to specified value"),
                )
                .arg(
                    Arg::new("to_id")
                        .long("to")
                        .value_name("ID")
                        .help("List tasks with ID less than or equal to specified value"),
                )
                .arg(
                    Arg::new("search")
                        .long("search")
                        .value_name("KEYWORD")
                        .help("Search tasks by keyword in task content"),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(clap::ArgAction::SetTrue)
                        .help("Output tasks in JSON format"),
                ),
        )
        .subcommand(
            Command::new("done")
                .about("Mark task as done")
                .arg(Arg::new("id").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove task")
                .arg(Arg::new("id").required(true)),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a task")
                .arg(Arg::new("id").required(true))
                .arg(
                    Arg::new("task")
                        .long("task")
                        .short('t')
                        .help("The new description of the task"),
                )
                .arg(
                    Arg::new("date")
                        .long("date")
                        .short('d')
                        .help("The new date of the task"),
                ),
        )
        .subcommand(Command::new("prompt-today").about("Print status icons for prompt"))
        .subcommand(Command::new("review").about("Review tasks overdue by more than 7 days"))
        .subcommand(
            Command::new("reuse")
                .about("Reuse an existing task by ID, optionally with a new date")
                .arg(Arg::new("id").required(true))
                .arg(
                    Arg::new("date")
                        .long("date")
                        .value_name("DATE")
                        .help("New date for the reused task (YYYY-MM-DD)"),
                ),
        )
        .subcommand(
            Command::new("init")
                .about("Print shell integration script")
                .arg(Arg::new("shell").required(true)),
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("add", sub)) => match sub.get_one::<String>("task") {
            Some(task) => cli::add(
                task.to_string(),
                sub.get_one::<String>("date").map(|s| s.to_string()),
            ),
            None => {
                eprintln!("Error: task is required");
                Ok(())
            }
        },
        Some(("list", sub)) => cli::list(
            sub.get_one::<String>("date").map(|s| s.to_string()),
            sub.get_flag("week"),
            sub.get_flag("month"),
            sub.get_flag("done"),
            sub.get_flag("pending"),
            sub.get_one::<String>("from_id")
                .and_then(|s| s.parse::<usize>().ok()),
            sub.get_one::<String>("to_id")
                .and_then(|s| s.parse::<usize>().ok()),
            sub.get_one::<String>("search").map(|s| s.to_lowercase()),
            sub.get_flag("json"),
        ),
        Some(("done", sub)) => match sub.get_one::<String>("id") {
            Some(id_str) => match id_str.parse::<usize>() {
                Ok(id) => cli::mark_done(id),
                Err(_) => {
                    eprintln!("Error: id must be a valid number");
                    Ok(())
                }
            },
            None => {
                eprintln!("Error: id is required");
                Ok(())
            }
        },
        Some(("rm", sub)) => match sub.get_one::<String>("id") {
            Some(id_str) => match id_str.parse::<usize>() {
                Ok(id) => cli::remove(id),
                Err(_) => {
                    eprintln!("Error: id must be a valid number");
                    Ok(())
                }
            },
            None => {
                eprintln!("Error: id is required");
                Ok(())
            }
        },
        Some(("edit", sub)) => match sub.get_one::<String>("id") {
            Some(id_str) => match id_str.parse::<usize>() {
                Ok(id) => cli::edit(
                    id,
                    sub.get_one::<String>("task").map(|s| s.to_string()),
                    sub.get_one::<String>("date").map(|s| s.to_string()),
                ),
                Err(_) => {
                    eprintln!("Error: id must be a valid number");
                    Ok(())
                }
            },
            None => {
                eprintln!("Error: id is required");
                Ok(())
            }
        },
        Some(("prompt-today", _)) => cli::prompt_today(),
        Some(("review", _)) => cli::review(),
        Some(("reuse", sub)) => match sub.get_one::<String>("id") {
            Some(id_str) => match id_str.parse::<usize>() {
                Ok(id) => cli::reuse(id, sub.get_one::<String>("date").map(|s| s.to_string())),
                Err(_) => {
                    eprintln!("Error: id must be a valid number");
                    Ok(())
                }
            },
            None => {
                eprintln!("Error: id is required");
                Ok(())
            }
        },
        Some(("init", sub)) => {
            if let Some(shell) = sub.get_one::<String>("shell") {
                shell::init_shell(shell);
                Ok(())
            } else {
                eprintln!("Error: shell is required");
                Ok(())
            }
        }
        _ => {
            println!("Use `td --help` to see available commands.");
            Ok(())
        }
    };

    if let Err(e) = task_store::TaskStore::save_to_disk() {
        eprintln!("Error saving tasks to disk: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
