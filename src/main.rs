mod commands;
use clap::{App, Arg, SubCommand};
use commands::task;
use std::{
    path::Path,
    env
};

fn main() -> Result<(), commands::error::Error> {
    let matches = App::new("rustask")
        .version("0.1")
        .author("bsdinis <baltasar.dinis@tecnico.ulisboa.pt>")
        .about("Task Manager")
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("task file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("l").help("List tasks"))
        .subcommand(SubCommand::with_name("list").help("List tasks"))
        .subcommand(SubCommand::with_name("la").help("List all tasks"))
        .subcommand(SubCommand::with_name("listall").help("List all tasks"))
        .subcommand(
            SubCommand::with_name("a")
                .help("Add a task")
                .arg(
                    Arg::with_name("task")
                        .help("the task to be added")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority for the task")
                        .takes_value(true)
                        .short("-p"),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .help("Add a task")
                .arg(
                    Arg::with_name("task")
                        .help("the task to be added")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority for the task")
                        .takes_value(true)
                        .short("-p"),
                ),
        )
        .subcommand(
            SubCommand::with_name("d")
                .help("Conclude the task")
                .arg(Arg::with_name("task index").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("done")
                .help("Conclude the task")
                .arg(Arg::with_name("task index").index(1).required(true)),
        )
        .get_matches();

    if !matches.is_present("file") && env::var("RUSTASK_TASKFILE").is_err() {
        eprintln!("Could not find rustask file");
        eprintln!("Maybe set RUSTASK_TASKFILE env var or pass in -f flag");
    }

    let task_location = if matches.is_present("file") {
        matches.value_of("file").unwrap().to_string()
    } else {
        env::var("RUSTASK_TASKFILE").unwrap()
    }
    ;
    let path = Path::new(&task_location);

    match matches.subcommand_name() {
        Some("list") | Some("l") => commands::list(path)?,
        Some("listall") | Some("la") => commands::list_all(path)?,
        Some("add") | Some("a") => {
            let sub_matches = matches
                .subcommand_matches("add")
                .or(matches.subcommand_matches("a"))
                .unwrap();
            let task_descript = sub_matches
                .value_of("task")
                .unwrap()
                .parse::<String>()
                .unwrap();

            if let Some(priority_str) = sub_matches.value_of("priority") {
                if let Ok(priority) = priority_str.parse::<task::Priority>() {
                    let task = task::make_task(task_descript, priority);
                    commands::add_task(path, task)?;
                } else {
                    eprintln!("{} is an invalid priority value", priority_str);
                    eprintln!("choose on of: urgent, high, normal, low, note")
                }
            } else {
                let task = task::make_default_task(task_descript);
                commands::add_task(path, task)?;
            }
        }
        Some("done") | Some("d") => {
            let sub_matches = matches
                .subcommand_matches("done")
                .or(matches.subcommand_matches("d"))
                .unwrap();

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                commands::remove_task(path, idx)?;
            } else {
                eprintln!("error: Refer to the task done by its id");
            }
        }
        _ => {
            eprintln!("No subcommand supplied");
        }
    };

    Ok(())
}
