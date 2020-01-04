// task.rs
//
// define task type

//use serde::{ Serialize, Deserialize };
//use chrono::prelude::*;
use std::fmt;
use colored::*;

pub enum Priority {
    Urgent,
    High,
    Normal,
    Low,
    Note
}

//#[derive(Serialize, Deserialize)]
pub struct Task {
    pub descript: String,
    pub priority: Option<Priority>,
    //deadline: Option<DateTime<Utc>>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.priority {
            Some(Priority::Urgent) => write!(f, "{}", self.descript.red().bold()),
            Some(Priority::High) => write!(f, "{}", self.descript.red()),
            Some(Priority::Normal) => write!(f, "{}", self.descript.yellow()),
            Some(Priority::Low) => write!(f, "{}", self.descript.green()),
            Some(Priority::Note) => write!(f, "{}", self.descript.cyan()),
            None => write!(f, "{}", self.descript.bold()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn urgent_task_display() {
        let task =  Task{
                        descript: "urgent task".to_string(),
                        priority: Some(Priority::Urgent)
        };
        assert_eq!(
            format!("Urgent Task: {}", task),
            format!("Urgent Task: {}", "urgent task".red().bold()),
            );
    }

    #[test]
    fn high_task_display() {
        let task =  Task{
                        descript: "high task".to_string(),
                        priority: Some(Priority::High)
        };
        assert_eq!(
            format!("High Task: {}", task),
            format!("High Task: {}", "high task".red()),
            );
    }

    #[test]
    fn normal_task_display() {
        let task =  Task{
                        descript: "normal task".to_string(),
                        priority: Some(Priority::Normal)
        };
        assert_eq!(
            format!("Normal Task: {}", task),
            format!("Normal Task: {}", "normal task".yellow()),
            );
    }

    #[test]
    fn low_task_display() {
        let task =  Task{
                        descript: "low task".to_string(),
                        priority: Some(Priority::Low)
        };
        assert_eq!(
            format!("Low Task: {}", task),
            format!("Low Task: {}", "low task".green()),
            );
    }

    #[test]
    fn note_display() {
        let task =  Task{
                        descript: "note".to_string(),
                        priority: Some(Priority::Note)
        };
        assert_eq!(
            format!("Note: {}", task),
            format!("Note: {}", "note".cyan()),
            );
    }
}
