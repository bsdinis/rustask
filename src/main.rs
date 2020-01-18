mod commands;
use clap::{App, Arg, SubCommand};
use commands::task;
use std::{
    path::Path,
    env
};

fn main() -> Result<(), commands::error::Error> {
    let matches = App::new("rustask")
        .version("0.5")
        .author("bsdinis <baltasar.dinis@tecnico.ulisboa.pt>")
        .about("Task Manager")
        .arg(
            Arg::with_name("file")
                .short("-f")
                .help("task file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("list")
                .aliases(&["l"])
                .help("List tasks")
                .arg(
                    Arg::with_name("project")
                        .help("project to be listed")
                        .index(1),
                )
            )
        .subcommand(
            SubCommand::with_name("listall")
                .aliases(&["la"])
                .help("List all tasks")
                .arg(
                    Arg::with_name("project")
                        .help("project to be listed")
                        .index(1),
                )
            )
        .subcommand(
            SubCommand::with_name("rename")
                .aliases(&["r"])
                .help("Rename a project")
                .arg(Arg::with_name("project")
                        .help("project to rename")
                        .index(1)
                        .required(true)
                )
                .arg(Arg::with_name("name")
                        .help("new name")
                        .index(2)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("add")
                .aliases(&["a"])
                .help("Add a task")
                .arg(
                    Arg::with_name("project")
                        .help("project to assign the task to")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("task")
                        .help("the task to be added")
                        .index(2)
                        .required(true)
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority (urgency) for the task")
                        .takes_value(true)
                        .short("-p"),
                )
        )
        .subcommand(
            SubCommand::with_name("done")
                .aliases(&["d"])
                .help("Conclude the task")
                .arg(Arg::with_name("project")
                        .help("project where the task is assigned to")
                        .index(1)
                        .required(true)
                )
                .arg(Arg::with_name("task index").index(2).required(true))
        )
        .subcommand(
            SubCommand::with_name("move")
                .aliases(&["m"])
                .help("Move a task between projects")
                .arg(Arg::with_name("old project")
                        .help("project where the task is")
                        .index(1)
                        .required(true)
                )
                .arg(Arg::with_name("id")
                        .help("id of the task being moved")
                        .index(2)
                        .required(true)
                )
                .arg(Arg::with_name("new project")
                        .help("new project for the task")
                        .index(3)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("edit")
                .aliases(&["e"])
                .help("Change a task")
                .arg(Arg::with_name("project")
                        .help("project to where the task assigned to")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::with_name("task index")
                        .help("the index of the task to be changed")
                        .index(2)
                        .required(true)
                )
                .arg(
                    Arg::with_name("descript")
                        .help("the new description")
                        .index(3)
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority (urgency) for the task")
                        .takes_value(true)
                        .short("-p"),
                )
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
        Some("list") => {
            let sub_matches = matches.subcommand_matches("list").unwrap();
            let project = sub_matches
                .value_of("project")
                .and_then(|s| s.to_string().parse::<String>().ok());

            commands::list(path, project)?
        },
        Some("listall") => {
            let sub_matches = matches.subcommand_matches("listall").unwrap();
            let project = sub_matches
                .value_of("project")
                .and_then(|s| s.to_string().parse::<String>().ok());

            commands::list_all(path, project)?
        },
        Some("rename") => {
            let sub_matches = matches.subcommand_matches("rename").unwrap();
            let project = sub_matches
                .value_of("project")
                .unwrap()
                .parse::<String>()
                .unwrap();

            let name = sub_matches
                .value_of("name")
                .unwrap()
                .parse::<String>()
                .unwrap();

            commands::rename(path, project, name)?
        },
        Some("move") => {
            let sub_matches = matches.subcommand_matches("move").unwrap();
            let old_project = sub_matches
                .value_of("old project")
                .unwrap()
                .parse::<String>()
                .unwrap();

            let id = sub_matches
                .value_of("id")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let new_project = sub_matches
                .value_of("new project")
                .unwrap()
                .parse::<String>()
                .unwrap();

            commands::move_task(path, old_project, id, new_project)?
        },
        Some("add") => {
            let sub_matches = matches.subcommand_matches("add").unwrap();
            let task_descript = sub_matches
                .value_of("task")
                .unwrap()
                .parse::<String>()
                .unwrap();

            let project = sub_matches
                .value_of("project")
                .unwrap_or("")
                .parse::<String>()
                .unwrap();

            if let Some(priority_str) = sub_matches.value_of("priority") {
                if let Ok(priority) = priority_str.parse::<task::Priority>() {
                    let task = task::Task::new(task_descript, Some(priority));
                    commands::add_task(path, task, project)?;
                } else {
                    eprintln!("{} is an invalid priority value", priority_str);
                    eprintln!("choose on of: urgent, high, normal, low, note")
                }
            } else {
                let task = task::Task::new(task_descript, None);
                commands::add_task(path, task, project)?;
            }
        }
        Some("done") => {
            let sub_matches = matches.subcommand_matches("done").unwrap();
            let project = sub_matches
                .value_of("project")
                .unwrap_or("")
                .parse::<String>()
                .unwrap();

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                let task = commands::remove_task(path, idx, project)?;
                println!("finished task {}: {}", idx, task);
            } else {
                eprintln!("error: Refer to the task done by its id");
            }
        }
        Some("edit") => {
            let sub_matches = matches.subcommand_matches("edit").unwrap();
            let project = sub_matches
                .value_of("project")
                .unwrap_or("")
                .parse::<String>()
                .unwrap();

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                let task_descript = sub_matches.value_of("descript")
                    .and_then(|d_str| d_str.parse::<String>().ok());

                let priority = sub_matches.value_of("priority")
                    .and_then(|p_str| p_str.parse::<task::Priority>().ok());

                commands::edit_task(path, idx, project, task_descript, priority)?;
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
