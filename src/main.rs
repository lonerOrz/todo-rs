use clap::{Arg, Command};

mod cli;
mod model;
mod shell;

fn main() {
    let matches = Command::new("td")
        .version("0.1.0")
        .about("Minimalistic CLI Todo")
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(Arg::new("task").required(true))
                .arg(Arg::new("date")
                    .long("date")
                    .value_name("DATE")),
        )
        .subcommand(
            Command::new("list")
                .about("List tasks")
                .arg(Arg::new("date")
                    .long("date")
                    .value_name("DATE")),
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
        .subcommand(Command::new("count").about("Count today's unfinished tasks"))
        .subcommand(Command::new("prompt-today").about("Print status icons for prompt"))
        .subcommand(
            Command::new("init")
                .about("Print shell integration script")
                .arg(Arg::new("shell").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub)) => cli::add(
            sub.get_one::<String>("task").unwrap().to_string(),
            sub.get_one::<String>("date").map(|s| s.to_string()),
        ),
        Some(("list", sub)) => cli::list(
            sub.get_one::<String>("date").map(|s| s.to_string()),
        ),
        Some(("done", sub)) => cli::mark_done(
            sub.get_one::<String>("id").unwrap().parse().unwrap(),
        ),
        Some(("rm", sub)) => cli::remove(
            sub.get_one::<String>("id").unwrap().parse().unwrap(),
        ),
        Some(("count", _)) => cli::count_today(),
        Some(("prompt-today", _)) => cli::prompt_today(),
        Some(("init", sub)) => shell::init_shell(sub.get_one::<String>("shell").unwrap()),
        _ => println!("Use `td --help` to see available commands."),
    }
}
