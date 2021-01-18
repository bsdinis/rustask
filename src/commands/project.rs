// project.rs
//
// define project type
//
use crate::commands::error::RustaskError;
use crate::commands::task::Task;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    tasks: Vec<Task>,
}

impl Project {
    pub fn new(name: String) -> Project {
        Project {
            name,
            tasks: vec![],
        }
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn push(&mut self, t: Task) {
        self.tasks.push(t);
        self.tasks.sort();
    }

    pub fn remove(&mut self, id: usize) -> Result<Task, RustaskError> {
        if id >= self.tasks.len() {
            return Err(RustaskError::OutOfBounds(id));
        }
        Ok(self.tasks.remove(id))
    }

    pub fn edit<F>(&mut self, id: usize, transform: F) -> Result<(), RustaskError>
    where
        F: FnOnce(&mut Task),
    {
        if id >= self.tasks.len() {
            return Err(RustaskError::OutOfBounds(id));
        }

        self.tasks.get_mut(id).map(|task_ref| transform(task_ref));

        Ok(())
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} {}",
            self.name,
            self.tasks.len(),
            if self.tasks.len() == 1 {
                "task"
            } else {
                "tasks"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::task::TaskBuilder;

    #[test]
    fn create() {
        let p = Project::new(String::from("project"));
        assert_eq!(p.name, String::from("project"));
        assert_eq!(p.tasks.len(), 0);
    }

    #[test]
    fn rename() {
        let mut p = Project::new(String::from("project"));
        assert_eq!(p.name, String::from("project"));
        assert_eq!(p.tasks.len(), 0);
        p.rename(String::from("another_project"));
        assert_eq!(p.name, String::from("another_project"));
        assert_eq!(p.tasks.len(), 0);
    }

    #[test]
    fn push() {
        let mut p = Project::new(String::from("project"));
        let task = TaskBuilder::new("task".to_string()).build();
        p.push(task.clone());

        assert_eq!(p.tasks.len(), 1);
        assert_eq!(p.tasks[0], task);
    }

    #[test]
    fn remove_out_of_bounds() {
        let mut p = Project::new(String::from("project"));

        assert_eq!(p.remove(0).unwrap_err(), RustaskError::OutOfBounds(0));
    }

    #[test]
    fn remove() {
        let mut p = Project::new(String::from("project"));
        let task = TaskBuilder::new("task".to_string()).build();
        p.push(task.clone());

        assert_eq!(p.remove(0).unwrap(), task);
    }

    #[test]
    fn edit_out_of_bounds() {
        let mut p = Project::new(String::from("project"));
        let task = TaskBuilder::new("task".to_string()).build();
        p.push(task);

        assert_eq!(p.edit(1, |_| {}).unwrap_err(), RustaskError::OutOfBounds(1));
    }

    #[test]
    fn edit() {
        let mut p = Project::new(String::from("project"));
        let task = TaskBuilder::new("task".to_string()).build();
        p.push(task.clone());

        assert_eq!(p.edit(0, |_| {}), Ok(()));
        assert_eq!(p.tasks[0], task);
    }

    #[test]
    fn tasks() {
        let mut p = Project::new(String::from("project"));
        assert_eq!(p.tasks().len(), 0);
        let task = TaskBuilder::new("task".to_string()).build();
        p.push(task.clone());
        assert_eq!(p.tasks().len(), 1);
        assert_eq!(p.tasks()[0], task);
    }
}
