// storage.rs
//
// store a list of tasks in a file

use std::{fs, path::Path};

use crate::commands::error;
use crate::commands::task::Task;
use serde_json;

pub fn store_tasks(path: &Path, tasks: &Vec<Task>) -> Result<(), error::Error> {
    let f = fs::File::create(path)?;
    serde_json::to_writer(f, tasks).unwrap();
    Ok(())
}

pub fn load_tasks(path: &Path) -> Result<Vec<Task>, error::Error> {
    let f = fs::File::open(path)?;
    let mut v: Vec<Task> = serde_json::from_reader(f).unwrap();
    v.sort();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::task;
    use std::path::Path;
    #[test]
    fn store_load() {
        let path = Path::new("test_file");
        let mut tasks = vec![
            task::Task::new("urgent".to_string(), Some(task::Priority::Urgent)),
            task::Task::new("high2".to_string(), Some(task::Priority::High)),
            task::Task::new("normal".to_string(), Some(task::Priority::Normal)),
            task::Task::new("low".to_string(), Some(task::Priority::Low)),
            task::Task::new("note".to_string(), Some(task::Priority::Note)),
            task::Task::new("default".to_string(), None),
        ];
        tasks.sort();

        store_tasks(&path, &tasks).unwrap();
        let new_tasks = load_tasks(&path).unwrap();

        assert_eq!(tasks, new_tasks);
        fs::remove_file(path).unwrap();
    }
}
