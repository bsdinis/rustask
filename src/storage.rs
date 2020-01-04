// storage.rs
//
// store a list of tasks in a file

use std::{
    path::Path,
    io,
    fs,
};

use crate::task::Task;
use serde_yaml;


pub fn store_tasks(path : &Path, tasks: &Vec<Task>) -> io::Result<()> {
    let f = fs::File::create(path)?;
    Ok(serde_yaml::to_writer(f, tasks).expect("failed to serialize"))
}

pub fn load_tasks(path : &Path) -> io::Result<Vec<Task>> {
    let f = fs::File::open(path)?;
    Ok(serde_yaml::from_reader(f).expect("failed to parse yaml"))
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs;
    use crate::task;
    use super::*;
    #[test]
    fn store_load() {
        let path = Path::new("test_file");
        let tasks = vec![
            task::make_task("urgent".to_string(), task::Priority::Urgent),
            task::make_task("high2".to_string(), task::Priority::High),
            task::make_task("normal".to_string(), task::Priority::Normal),
            task::make_task("low".to_string(), task::Priority::Low),
            task::make_task("note".to_string(), task::Priority::Note),
            task::make_default_task("default".to_string()),
        ];

        store_tasks(&path, &tasks).unwrap();
        let new_tasks = load_tasks(&path).unwrap();

        assert_eq!(tasks, new_tasks);
        fs::remove_file(path).unwrap();
    }
}
