// storage.rs
//
// store a list of tasks in a file

use std::{fs, path::Path};

use crate::commands::error::RustaskError;
use crate::commands::project::Project;
use serde_json;

pub fn store_tasks(path: &Path, tasks: &Vec<Project>) -> Result<(), RustaskError> {
    let f = fs::File::create(path)?;
    serde_json::to_writer(f, tasks).unwrap();
    Ok(())
}

pub fn load_tasks(path: &Path) -> Result<Vec<Project>, RustaskError> {
    let f = fs::File::open(path)?;
    let mut v: Vec<Project> = serde_json::from_reader(f)?;
    v.sort();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::project;
    use crate::commands::task;
    use std::path::Path;
    #[test]
    fn store_load() {
        let path = Path::new("test_file");
        let mut p = vec![
            project::Project::new("proj0".to_string()),
            project::Project::new("proj1".to_string()),
        ];

        p[0].push(
            task::TaskBuilder::new("urgent".to_string())
                .priority(task::Priority::Urgent)
                .build(),
        );
        p[0].push(
            task::TaskBuilder::new("high2".to_string())
                .priority(task::Priority::High)
                .deadline(task::now_deadline())
                .build(),
        );
        p[1].push(
            task::TaskBuilder::new("normal".to_string())
                .priority(task::Priority::Normal)
                .deadline(task::now_deadline())
                .build(),
        );
        p[1].push(
            task::TaskBuilder::new("low".to_string())
                .priority(task::Priority::Low)
                .deadline(task::now_deadline())
                .build(),
        );
        p[1].push(
            task::TaskBuilder::new("note".to_string())
                .priority(task::Priority::Note)
                .deadline(task::now_deadline())
                .build(),
        );
        p[1].push(
            task::TaskBuilder::new("another_default".to_string())
                .deadline(task::now_deadline())
                .build(),
        );
        p[1].push(task::TaskBuilder::new("default".to_string()).build());

        store_tasks(&path, &p).unwrap();
        let new_p = load_tasks(&path).unwrap();

        assert_eq!(p, new_p);
        fs::remove_file(path).unwrap();
    }
}
