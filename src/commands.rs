use std::path::Path;

pub mod error;
pub mod project;
mod storage;
pub mod task;

use project::Project;
use task::Task;

fn list_filter<F>(path: &Path, project: Option<String>, op: F) -> Result<(), error::RustaskError>
where
    F: Fn(&usize, &Task) -> bool,
{
    let projects = storage::load_tasks(&path)?;
    match project {
        None => {
            let projs = storage::load_tasks(&path)?;
            for (i, proj) in projs.iter().enumerate() {
                println!("{}", proj);
                proj.tasks()
                    .iter()
                    .enumerate()
                    .filter(|(i, t)| op(i, *t))
                    .for_each(|(idx, t)| println!("[{}]: {}", idx, t));
                if i != projs.len() - 1 {
                    println!("");
                }
            }
            Ok(())
        }
        Some(name) => {
            if let Some(idx) = projects.iter().position(|p| p.name == name) {
                println!("{}", projects[idx]);
                projects[idx]
                    .tasks()
                    .iter()
                    .enumerate()
                    .filter(|(i, t)| op(i, *t))
                    .for_each(|(idx, t)| println!("[{}]: {}", idx, t));
                Ok(())
            } else {
                Err(error::RustaskError::ProjectNotFound(name))
            }
        }
    }
}

/// List the tasks in the path given
pub fn list_all(path: &Path, project: Option<String>) -> Result<(), error::RustaskError> {
    list_filter(path, project, |_a, _b| true)
}

/// List the tasks in the path given (depends on priority)
pub fn list(path: &Path, project: Option<String>) -> Result<(), error::RustaskError> {
    list_filter(path, project, |_, t| t.choose())
}

/// Renames a project if it exists and if the other name is not taken
pub fn rename(path: &Path, project: String, name: String) -> Result<(), error::RustaskError> {
    let mut projs = storage::load_tasks(&path)?;
    if let Some(idx) = projs.iter().position(|p| p.name == project) {
        if projs.iter().any(|p| p.name == name) {
            return Err(error::RustaskError::ProjectNameTaken(name));
        }
        projs[idx].rename(name);
    } else {
        return Err(error::RustaskError::ProjectNotFound(project));
    }
    storage::store_tasks(&path, &projs)
}

/// Add a new task
pub fn add_task(path: &Path, task: Task, name: String) -> Result<(), error::RustaskError> {
    let mut projs = storage::load_tasks(&path)?;
    if let Some(idx) = projs.iter().position(|p| p.name == name) {
        projs[idx].push(task);
    } else {
        let mut p = Project::new(name);
        p.push(task);
        projs.push(p);
        projs.sort();
    }

    storage::store_tasks(&path, &projs)
}

/// Remove a task
pub fn remove_task(path: &Path, id: usize, name: String) -> Result<Task, error::RustaskError> {
    let mut projs = storage::load_tasks(&path)?;
    let idx_res = projs.iter().position(|p| p.name == name);
    if idx_res.is_none() {
        return Err(error::RustaskError::ProjectNotFound(name));
    }

    let idx = idx_res.unwrap();

    let task = projs[idx].remove(id)?;
    if projs[idx].len() == 0 {
        projs.remove(idx);
    }
    storage::store_tasks(&path, &projs)?;
    Ok(task)
}

pub fn move_task(
    path: &Path,
    old_project: String,
    id: usize,
    new_project: String,
) -> Result<(), error::RustaskError> {
    let task = remove_task(path, id, old_project)?;
    add_task(path, task, new_project)
}

/// Edit a task
pub fn edit_task(
    path: &Path,
    id: usize,
    name: String,
    description: Option<String>,
    priority: Option<task::Priority>,
    deadline: Option<task::Deadline>,
) -> Result<(), error::RustaskError> {
    let mut projs = storage::load_tasks(&path)?;
    let idx_res = projs.iter().position(|p| p.name == name);
    if idx_res.is_none() {
        return Err(error::RustaskError::ProjectNotFound(name));
    }

    let idx = idx_res.unwrap();
    projs[idx].edit(id, |mut task| {
        if let Some(d) = description {
            task.description = d;
        }
        if let Some(p) = priority {
            task.priority = Some(p);
        }
        if let Some(d) = deadline {
            task.deadline = Some(d);
        }
    })?;
    storage::store_tasks(&path, &projs)?;
    Ok(())
}
