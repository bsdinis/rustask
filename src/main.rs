mod commands;
use clap::{App, Arg, SubCommand};
use commands::task;
use std::{env, path::Path};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let matches = App::new("rustask")
        .version("0.9")
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
                ),
        )
        .subcommand(
            SubCommand::with_name("listall")
                .aliases(&["la"])
                .help("List all tasks")
                .arg(
                    Arg::with_name("project")
                        .help("project to be listed")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("rename")
                .aliases(&["r"])
                .help("Rename a project")
                .arg(
                    Arg::with_name("project")
                        .help("project to rename")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("name")
                        .help("new name")
                        .index(2)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .aliases(&["a"])
                .help("Add a task")
                .arg(
                    Arg::with_name("project")
                        .help("project to assign the task to")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("task")
                        .help("the task to be added")
                        .index(2)
                        .required(true),
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority (urgency) for the task")
                        .takes_value(true)
                        .short("-p"),
                )
                .arg(
                    Arg::with_name("deadline")
                        .help("deadline of the task")
                        .takes_value(true)
                        .short("-d"),
                ),
        )
        .subcommand(
            SubCommand::with_name("done")
                .aliases(&["d"])
                .help("Conclude the task")
                .arg(
                    Arg::with_name("project")
                        .help("project where the task is assigned to")
                        .index(1)
                        .required(true),
                )
                .arg(Arg::with_name("task index").index(2).required(true)),
        )
        .subcommand(
            SubCommand::with_name("move")
                .aliases(&["m"])
                .help("Move a task between projects")
                .arg(
                    Arg::with_name("old project")
                        .help("project where the task is")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("id")
                        .help("id of the task being moved")
                        .index(2)
                        .required(true),
                )
                .arg(
                    Arg::with_name("new project")
                        .help("new project for the task")
                        .index(3)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .aliases(&["e"])
                .help("Change a task")
                .arg(
                    Arg::with_name("project")
                        .help("project to where the task assigned to")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("task index")
                        .help("the index of the task to be changed")
                        .index(2)
                        .required(true),
                )
                .arg(
                    Arg::with_name("descript")
                        .help("the new description")
                        .index(3),
                )
                .arg(
                    Arg::with_name("priority")
                        .help("priority (urgency) for the task")
                        .takes_value(true)
                        .short("-p"),
                )
                .arg(
                    Arg::with_name("deadline")
                        .help("deadline of the task")
                        .takes_value(true)
                        .short("-d"),
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
        env::var("RUSTASK_TASKFILE")?
    };
    let path = Path::new(&task_location);

    match matches.subcommand_name() {
        Some("list") => {
            let sub_matches = matches.subcommand_matches("list").unwrap();
            let project = sub_matches
                .value_of("project")
                .and_then(|s| s.to_string().parse::<String>().ok());

            commands::list(path, project)?
        }
        Some("rename") => {
            let sub_matches = matches.subcommand_matches("rename").unwrap();
            let project = sub_matches.value_of("project").unwrap().parse::<String>()?;

            let name = sub_matches.value_of("name").unwrap().parse::<String>()?;

            commands::rename(path, project, name)?
        }
        Some("move") => {
            let sub_matches = matches.subcommand_matches("move").unwrap();
            let old_project = sub_matches
                .value_of("old project")
                .unwrap()
                .parse::<String>()?;

            let id = sub_matches.value_of("id").unwrap().parse::<usize>()?;

            let new_project = sub_matches
                .value_of("new project")
                .unwrap()
                .parse::<String>()?;

            commands::move_task(path, old_project, id, new_project)?
        }
        Some("add") => {
            let sub_matches = matches.subcommand_matches("add").unwrap();
            let task_descript = sub_matches.value_of("task").unwrap().parse::<String>()?;

            let project = sub_matches
                .value_of("project")
                .unwrap_or("")
                .parse::<String>()?;

            let priority = sub_matches
                .value_of("priority")
                .and_then(|s| s.parse::<task::Priority>().ok());
            let deadline = if let Some(p_str) = sub_matches.value_of("deadline") {
                Some(task::parse_deadline(p_str)?)
            } else {
                None
            };

            let task_b = task::TaskBuilder::new(task_descript);
            let task_b = if let Some(p) = priority {
                task_b.priority(p)
            } else {
                task_b
            };
            let task_b = if let Some(d) = deadline {
                task_b.deadline(d)
            } else {
                task_b
            };
            commands::add_task(path, task_b.build(), project.clone())?;
            commands::list_all(path, Some(project))?;
        }
        Some("done") => {
            let sub_matches = matches.subcommand_matches("done").unwrap();
            let project = sub_matches
                .value_of("project")
                .unwrap_or("")
                .parse::<String>()?;

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                let task = commands::remove_task(path, idx, project.clone())?;
                println!("finished task {}: {}", idx, task);
                commands::list_all(path, Some(project))?
            } else {
                eprintln!("error: Refer to the task done by its id");
            }
        }
        Some("edit") => {
            let sub_matches = matches.subcommand_matches("edit").unwrap();
            let project = sub_matches
                .value_of("project")
                .unwrap_or("")
                .parse::<String>()?;

            if let Ok(idx) = sub_matches.value_of("task index").unwrap().parse::<usize>() {
                let task_descript = sub_matches
                    .value_of("descript")
                    .and_then(|d_str| d_str.parse::<String>().ok());

                let priority = sub_matches
                    .value_of("priority")
                    .and_then(|p_str| p_str.parse::<task::Priority>().ok());

                let deadline = if let Some(p_str) = sub_matches.value_of("deadline") {
                    Some(task::parse_deadline(p_str)?)
                } else {
                    None
                };

                commands::edit_task(
                    path,
                    idx,
                    project.clone(),
                    task_descript,
                    priority,
                    deadline,
                )?;
                commands::list_all(path, Some(project))?
            } else {
                eprintln!("error: Refer to the task done by its id");
            }
        }
        Some("listall") => {
            let sub_matches = matches.subcommand_matches("listall").unwrap();
            let project = sub_matches
                .value_of("project")
                .and_then(|s| s.to_string().parse::<String>().ok());

            commands::list_all(path, project)?
        }
        _ => commands::list_all(path, None)?,
    };

    Ok(())
}
