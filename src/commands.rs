use std::path::Path;

pub mod error;
mod storage;
pub mod task;
pub mod project;

use task::{task_f, Task};
use project::Project;

fn list_filter<F>(path: &Path, op: F) -> Result<(), error::Error>
where
    F: Fn(&usize, &Task) -> bool,
{
    storage::load_tasks(&path)?
        .iter()
        .enumerate()
        .filter(|(i, t)| op(i, *t))
        .for_each(|(idx, t)| println!("[{}]: {}", idx, t));
    Ok(())
}

/// List the tasks in the path given
pub fn list_all(path: &Path) -> Result<(), error::Error> {
    list_filter(path, |_a, _b| true)
}

/// List the tasks in the path given (depends on priority)
pub fn list(path: &Path) -> Result<(), error::Error> {
    list_filter(path, |_, t| task_f(&t))
}

/// Add a new task
pub fn add_task(path: &Path, task: Task) -> Result<(), error::Error> {
    let mut tasks = storage::load_tasks(&path)?;
    tasks.push(task);
    storage::store_tasks(&path, &tasks)
}

/// Remove a task
pub fn remove_task(path: &Path, id: usize) -> Result<Task, error::Error> {
    let mut tasks = storage::load_tasks(&path)?;
    if id >= tasks.len() {
        return Err(error::Error::OutOfBounds(id));
    }
    let task = tasks.remove(id);
    storage::store_tasks(&path, &tasks)?;
    Ok(task)
}

/// Edit a task
pub fn edit_task(path: &Path, id: usize, desc: Option<String>, prio: Option<task::Priority>) -> Result<(), error::Error> {
    let mut tasks = storage::load_tasks(&path)?;
    if id >= tasks.len() {
        return Err(error::Error::OutOfBounds(id));
    }

    tasks[id] = Task::new(desc.unwrap_or(tasks[id].descript.clone()),
                         prio.or(tasks[id].priority.clone()));
    storage::store_tasks(&path, &tasks)?;
    Ok(())
}
