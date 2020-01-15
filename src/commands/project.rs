// project.rs
//
// define project type

use serde::{Deserialize, Serialize};
use std::fmt;
use super::task;
use super::error;

#[derive(Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    tasks: Vec<task::Task>,
}

impl Project {
    pub fn new(name: String) -> Project {
        Project { name: name, tasks: vec![] }
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn push(&mut self, t: task::Task) {
        self.tasks.push(t);
        self.tasks.sort();
    }

    pub fn remove(&mut self, id: usize) -> Result<task::Task, error::Error> {
        if id >= self.tasks.len() {
            return Err(error::Error::OutOfBounds(id));
        }
        Ok(self.tasks.remove(id))
    }

    pub fn edit(&mut self, id: usize, desc: Option<String>, priority: Option<task::Priority>) -> Result<(), error::Error> {
        if id >= self.tasks.len() {
            return Err(error::Error::OutOfBounds(id));
        }

        self.tasks[id] = task::Task::new(desc.unwrap_or(self.tasks[id].descript.clone()),
                             priority.or(self.tasks[id].priority.clone()));

        Ok(())
    }

    pub fn tasks(&self) -> &Vec<task::Task> {
        &self.tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} {}", self.name, self.tasks.len(), if self.tasks.len() == 1 {"task"} else {"tasks"})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::task::Task;
    use crate::commands::error;

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
        p.push(Task::new(String::from("task"), None));

        assert_eq!(p.tasks.len(), 1);
        assert_eq!(p.tasks[0], Task::new(String::from("task"), None));
    }

    #[test]
    fn remove_out_of_bounds() {
        let mut p = Project::new(String::from("project"));

        assert_eq!(p.remove(0).unwrap_err(), error::Error::OutOfBounds(0));
    }

    #[test]
    fn remove() {
        let mut p = Project::new(String::from("project"));
        p.push(Task::new(String::from("task"), None));

        assert_eq!(p.remove(0).unwrap(), Task::new(String::from("task"), None));
    }

    #[test]
    fn edit_out_of_bounds() {
        let mut p = Project::new(String::from("project"));
        p.push(Task::new(String::from("task"), None));

        assert_eq!(p.edit(1, None, None).unwrap_err(), error::Error::OutOfBounds(1));
    }

    #[test]
    fn edit() {
        let mut p = Project::new(String::from("project"));
        p.push(Task::new(String::from("task"), None));

        assert_eq!(p.edit(0, None, None), Ok(()));
        assert_eq!(p.tasks[0], Task::new(String::from("task"), None));
    }

    #[test]
    fn tasks() {
        let mut p = Project::new(String::from("project"));
        assert_eq!(p.tasks().len(), 0);
        p.push(Task::new(String::from("task"), None));
        assert_eq!(p.tasks().len(), 1);
        assert_eq!(p.tasks()[0], Task::new(String::from("task"), None));
    }
}
