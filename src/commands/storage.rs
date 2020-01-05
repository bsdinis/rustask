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
            task::make_task("urgent".to_string(), task::Priority::Urgent),
            task::make_task("high2".to_string(), task::Priority::High),
            task::make_task("normal".to_string(), task::Priority::Normal),
            task::make_task("low".to_string(), task::Priority::Low),
            task::make_task("note".to_string(), task::Priority::Note),
            task::make_default_task("default".to_string()),
        ];
        tasks.sort();

        store_tasks(&path, &tasks).unwrap();
        let new_tasks = load_tasks(&path).unwrap();

        assert_eq!(tasks, new_tasks);
        fs::remove_file(path).unwrap();
    }
}
