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
        Some(("add", sub)) => {
            let task = sub.get_one::<String>("task")
                .expect("task is required")
                .to_string();
            cli::add(task, sub.get_one::<String>("date").map(|s| s.to_string()))
        },
        Some(("list", sub)) => cli::list(
            sub.get_one::<String>("date").map(|s| s.to_string()),
            sub.get_flag("week"),
            sub.get_flag("month"),
        ),
        Some(("done", sub)) => {
            let id_str = sub.get_one::<String>("id")
                .expect("id is required");
            let id: usize = id_str.parse()
                .expect("id must be a valid number");
            cli::mark_done(id)
        }
        Some(("rm", sub)) => {
            let id_str = sub.get_one::<String>("id")
                .expect("id is required");
            let id: usize = id_str.parse()
                .expect("id must be a valid number");
            cli::remove(id)
        }
        Some(("edit", sub)) => {
            let id_str = sub.get_one::<String>("id")
                .expect("id is required");
            let id: usize = id_str.parse()
                .expect("id must be a valid number");
            cli::edit(
                id,
                sub.get_one::<String>("task").map(|s| s.to_string()),
                sub.get_one::<String>("date").map(|s| s.to_string()),
            )
        },
        Some(("prompt-today", _)) => cli::prompt_today(),
        Some(("review", _)) => cli::review(),
        Some(("reuse", sub)) => {
            let id_str = sub.get_one::<String>("id")
                .expect("id is required");
            let id: usize = id_str.parse()
                .expect("id must be a valid number");
            cli::reuse(
                id,
                sub.get_one::<String>("date").map(|s| s.to_string()),
            )
        },
        Some(("init", sub)) => {
            let shell = sub.get_one::<String>("shell")
                .expect("shell is required");
            shell::init_shell(shell);
            Ok(())
        },
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