// storage.rs
//
// store a list of tasks in a file

use std::{fs, path::Path};

use crate::commands::error;
use crate::commands::project::Project;
use serde_json;

pub fn store_tasks(path: &Path, tasks: &Vec<Project>) -> Result<(), error::Error> {
    let f = fs::File::create(path)?;
    serde_json::to_writer(f, tasks).unwrap();
    Ok(())
}

pub fn load_tasks(path: &Path) -> Result<Vec<Project>, error::Error> {
    let f = fs::File::open(path)?;
    let mut v: Vec<Project> = serde_json::from_reader(f).unwrap();
    v.sort();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::task;
    use crate::commands::project;
    use std::path::Path;
    #[test]
    fn store_load() {
        let path = Path::new("test_file");
        let mut p = vec![
            project::Project::new("proj0".to_string()),
            project::Project::new("proj1".to_string())
        ];

        p[0].push(task::Task::new("urgent".to_string(), Some(task::Priority::Urgent)));
        p[0].push(task::Task::new("high2".to_string(), Some(task::Priority::High)));
        p[1].push(task::Task::new("normal".to_string(), Some(task::Priority::Normal)));
        p[1].push(task::Task::new("low".to_string(), Some(task::Priority::Low)));
        p[1].push(task::Task::new("note".to_string(), Some(task::Priority::Note)));
        p[1].push(task::Task::new("default".to_string(), None));

        store_tasks(&path, &p).unwrap();
        let new_p = load_tasks(&path).unwrap();

        assert_eq!(p, new_p);
        fs::remove_file(path).unwrap();
    }
}
