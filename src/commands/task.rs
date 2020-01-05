// task.rs
//
// define task type

//use chrono::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Urgent,
    High,
    Normal,
    Low,
    Note,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Task {
    pub descript: String,
    pub priority: Option<Priority>,
    //deadline: Option<DateTime<Utc>>,
}

pub fn make_task(description: String, priority: Priority) -> Task {
    Task {
        descript: description,
        priority: Some(priority),
    }
}

pub fn make_default_task(description: String) -> Task {
    Task {
        descript: description,
        priority: None,
    }
}

/// Filters a task, based on the priority
///
/// # Examples
///
/// ```
/// use rustask::commands::task::*;
/// let task = make_task("task".to_string(), Priority::Urgent);
/// assert_eq!(task_f(&task), true);
/// ```
pub fn task_f(task: &Task) -> bool {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let priority = task.priority.as_ref().unwrap_or(&Priority::Normal);

    match priority {
        Priority::Urgent => true,
        Priority::High => true,
        Priority::Normal => rng.gen_bool(1. / 3.),
        Priority::Low => rng.gen_bool(1. / 5.),
        Priority::Note => rng.gen_bool(1. / 8.),
    }
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

pub struct ParsePriorityError {}

impl FromStr for Priority {
    type Err = ParsePriorityError;
    fn from_str(s: &str) -> Result<Self, ParsePriorityError> {
        match s.to_lowercase().as_str() {
            "urgent" => Ok(Priority::Urgent),
            "high" => Ok(Priority::High),
            "normal" => Ok(Priority::Normal),
            "low" => Ok(Priority::Low),
            "note" => Ok(Priority::Note),
            _ => Err(ParsePriorityError {}),
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_pri = self.priority.as_ref().unwrap_or(&Priority::Normal);
        let other_pri = other.priority.as_ref().unwrap_or(&Priority::Normal);
        if self_pri == other_pri {
            self.descript.cmp(&other.descript)
        } else {
            self_pri.cmp(&other_pri)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn urgent_task_display() {
        let task = make_task("urgent task".to_string(), Priority::Urgent);
        assert_eq!(
            format!("Urgent Task: {}", task),
            format!("Urgent Task: {}", "urgent task".red().bold()),
        );
    }

    #[test]
    fn high_task_display() {
        let task = make_task("high task".to_string(), Priority::High);
        assert_eq!(
            format!("High Task: {}", task),
            format!("High Task: {}", "high task".red()),
        );
    }

    #[test]
    fn normal_task_display() {
        let task = make_task("normal task".to_string(), Priority::Normal);
        assert_eq!(
            format!("Normal Task: {}", task),
            format!("Normal Task: {}", "normal task".yellow()),
        );
    }

    #[test]
    fn low_task_display() {
        let task = make_task("low task".to_string(), Priority::Low);
        assert_eq!(
            format!("Low Task: {}", task),
            format!("Low Task: {}", "low task".green()),
        );
    }

    #[test]
    fn note_display() {
        let task = make_task("note".to_string(), Priority::Note);
        assert_eq!(
            format!("Note: {}", task),
            format!("Note: {}", "note".cyan()),
        );
    }

    #[test]
    fn default_display() {
        let task = make_default_task("default".to_string());
        assert_eq!(
            format!("Default: {}", task),
            format!("Default: {}", "default".bold()),
        );
    }

    #[test]
    fn urgent_filter() {
        let task = make_task("task".to_string(), Priority::Urgent);
        assert_eq!(task_f(&task), true);
    }

    #[test]
    fn high_filter() {
        let task = make_task("task".to_string(), Priority::High);
        assert_eq!(task_f(&task), true);
    }
}
