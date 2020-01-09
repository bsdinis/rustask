mod commands;
use clap::{App, Arg, SubCommand};
use commands::task;
use std::{
    path::Path,
    env
};

fn main() -> Result<(), commands::error::Error> {
    let matches = App::new("rustask")
        .version("0.3")
        .author("bsdinis <baltasar.dinis@tecnico.ulisboa.pt>")
        .about("Task Manager")
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("task file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("list").aliases(&["l"]).help("List tasks"))
        .subcommand(SubCommand::with_name("listall").aliases(&["la"]).help("List all tasks"))
        .subcommand(
            SubCommand::with_name("add")
                .aliases(&["a"])
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
            SubCommand::with_name("done")
                .aliases(&["d"])
                .help("Conclude the task")
                .arg(Arg::with_name("task index").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .aliases(&["e"])
                .help("Change a task")
                .arg(
                    Arg::with_name("task index")
                        .help("the index of the task to be changed")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("descript")
                        .help("the new description")
                        .index(2)
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority for the task")
                        .takes_value(true)
                        .short("-p"),
                ),
        )
        .get_matches();

    if !matches.is_present("file") && env::var("RUSTASK_TASKFILE").is_err() {
        eprintln!("Could not find rustask file");
        eprintln!("Maybe set RUSTASK_TASKFILE env var or pass in -f flag");
        std::process::exit(1);
    }

    let task_location = if matches.is_present("file") {
        matches.value_of("file").unwrap().to_string()
    } else {
        env::var("RUSTASK_TASKFILE").unwrap()
    }
    ;
    let path = Path::new(&task_location);

    match matches.subcommand_name() {
        Some("list") => commands::list(path)?,
        Some("listall") => commands::list_all(path)?,
        Some("add") => {
            let sub_matches = matches.subcommand_matches("add").unwrap();
            let task_descript = sub_matches
                .value_of("task")
                .unwrap()
                .parse::<String>()
                .unwrap();

            if let Some(priority_str) = sub_matches.value_of("priority") {
                if let Ok(priority) = priority_str.parse::<task::Priority>() {
                    let task = task::Task::new(task_descript, Some(priority));
                    commands::add_task(path, task)?;
                } else {
                    eprintln!("{} is an invalid priority value", priority_str);
                    eprintln!("choose on of: urgent, high, normal, low, note")
                }
            } else {
                let task = task::Task::new(task_descript, None);
                commands::add_task(path, task)?;
            }
        }
        Some("done") => {
            let sub_matches = matches.subcommand_matches("done").unwrap();

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                let task = commands::remove_task(path, idx)?;
                println!("finished task {}: {}", idx, task);
            } else {
                eprintln!("error: Refer to the task done by its id");
            }
        }
        Some("edit") => {
            let sub_matches = matches.subcommand_matches("edit").unwrap();

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                let task_descript = sub_matches.value_of("descript")
                    .and_then(|d_str| d_str.parse::<String>().ok());

                let priority = sub_matches.value_of("priority")
                    .and_then(|p_str| p_str.parse::<task::Priority>().ok());

                commands::edit_task(path, idx, task_descript, priority)?;
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
