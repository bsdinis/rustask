// task.rs
//
// define task type

//use chrono::prelude::*;
use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::{DateTime, Local, TimeZone};
use colored::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

pub type Deadline = DateTime<Local>;

#[derive(thiserror::Error, Debug)]
pub enum DeadlineParseError {
    #[error("failed to parse the string as a datetime: {}", .string)]
    ParseError {
        string: String,
        #[source]
        source: chrono::format::ParseError,
    },
    #[error("failed to assign a timezone")]
    TimezoneError,
}

pub fn parse_deadline(s: &str) -> Result<Deadline, DeadlineParseError> {
    const DATE_FMT: &str = "%F";
    const DATETIME_FMT: &str = "%F %H:%M";

    let naive: NaiveDateTime = NaiveDate::parse_from_str(s, DATE_FMT)
        .map(|date| date.and_hms(0, 0, 0))
        .or(NaiveDateTime::parse_from_str(s, DATETIME_FMT))
        .map_err(|e| DeadlineParseError::ParseError {
            string: s.to_string(),
            source: e,
        })?;

    Local
        .from_local_datetime(&naive)
        .earliest()
        .ok_or_else(|| DeadlineParseError::TimezoneError)
}

#[allow(unused)]
pub fn now_deadline() -> Deadline {
    Local::now()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Urgent,
    High,
    Normal,
    Low,
    Note,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Task {
    pub description: String,
    pub priority: Option<Priority>,
    pub deadline: Option<Deadline>,
}

pub struct TaskBuilder {
    description: String,
    priority: Option<Priority>,
    deadline: Option<Deadline>,
}

impl TaskBuilder {
    pub fn new(description: String) -> TaskBuilder {
        TaskBuilder {
            description,
            priority: None,
            deadline: None,
        }
    }

    pub fn priority(mut self, priority: Priority) -> TaskBuilder {
        self.priority = Some(priority);
        self
    }

    pub fn deadline(mut self, deadline: Deadline) -> TaskBuilder {
        self.deadline = Some(deadline);
        self
    }

    pub fn build(self) -> Task {
        Task {
            description: self.description,
            priority: self.priority,
            deadline: self.deadline,
        }
    }
}

impl Task {
    /// Whether to choose this task or not
    ///
    /// # Examples
    ///
    /// ```
    /// use rustask::commands::task::*;
    /// let task = TaskBuilder::new("task".to_string()).priority(Priority::Urgent).build();
    /// assert_eq!(task.choose(), true);
    /// ```
    pub fn choose(&self) -> bool {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        match self.priority.as_ref().unwrap_or(&Priority::Normal) {
            Priority::Urgent => true,
            Priority::High => true,
            Priority::Normal => {
                let prob = 1. / 3. + self.deadline_near();
                if prob < 1.0 {
                    rng.gen_bool(prob)
                } else {
                    true
                }
            }
            Priority::Low => {
                let prob = 1. / 5. + self.deadline_near();
                if prob < 1.0 {
                    rng.gen_bool(prob)
                } else {
                    true
                }
            }
            Priority::Note => {
                let prob = 1. / 8. + self.deadline_near();
                if prob < 1.0 {
                    rng.gen_bool(prob)
                } else {
                    true
                }
            }
        }
    }

    /// Whether there is a deadline near
    /// Yields a percentage, which can be read as an auxiliar priority level
    ///
    /// If a task is overdue or happening now, the percentage is 1.0
    /// If a task is more than a week in the future, the percentage is 0.0
    fn deadline_near(&self) -> f64 {
        if let Some(d) = self.deadline {
            let diff = d - now_deadline();
            if diff <= chrono::Duration::zero() {
                1.0
            } else if diff.num_weeks() > 0 {
                0.0
            } else {
                const MINUTES_IN_WEEK: f64 = (7 * 24 * 60) as f64;
                diff.num_minutes() as f64 / MINUTES_IN_WEEK
            }
        } else {
            0.0
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display_diff(diff: chrono::Duration) -> String {
            let abs = if diff < chrono::Duration::zero() {
                -diff
            } else {
                diff
            };
            format!(
                "{} {}",
                if diff < chrono::Duration::zero() {
                    "overdue by"
                } else {
                    "in"
                },
                if abs.num_weeks() > 0 {
                    format!(
                        "{} week{}",
                        abs.num_weeks(),
                        if abs.num_weeks() == 1 { "" } else { "s" }
                    )
                } else if abs.num_days() > 0 {
                    format!(
                        "{} day{}",
                        abs.num_days(),
                        if abs.num_days() == 1 { "" } else { "s" }
                    )
                } else if abs.num_hours() > 0 {
                    format!(
                        "{} hour{}",
                        abs.num_hours(),
                        if abs.num_hours() == 1 { "" } else { "s" }
                    )
                } else if abs.num_minutes() > 15 {
                    format!(
                        "{} minute{}",
                        abs.num_minutes(),
                        if abs.num_minutes() == 1 { "" } else { "s" }
                    )
                } else {
                    "moments".to_string()
                }
            )
        }

        if let Some(deadline) = self.deadline {
            write!(
                f,
                "{} [{}]",
                match &self.priority {
                    Some(Priority::Urgent) => self.description.on_red().bold().bright_white(),
                    Some(Priority::High) => self.description.red(),
                    Some(Priority::Normal) => self.description.yellow(),
                    Some(Priority::Low) => self.description.green(),
                    Some(Priority::Note) => self.description.cyan(),
                    None => self.description.bold(),
                },
                display_diff(deadline - now_deadline())
            )
        } else {
            write!(
                f,
                "{}",
                match &self.priority {
                    Some(Priority::Urgent) => self.description.on_red().bold().bright_white(),
                    Some(Priority::High) => self.description.red(),
                    Some(Priority::Normal) => self.description.yellow(),
                    Some(Priority::Low) => self.description.green(),
                    Some(Priority::Note) => self.description.cyan(),
                    None => self.description.bold(),
                }
            )
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
            self.description.cmp(&other.description)
        } else {
            self_pri.cmp(&other_pri)
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn urgent_task_display() {
        let task = TaskBuilder::new("urgent task".to_string())
            .priority(Priority::Urgent)
            .build();
        assert_eq!(
            format!("Urgent Task: {}", task),
            format!(
                "Urgent Task: {}",
                "urgent task".on_red().bold().bright_white()
            ),
        );
    }

    #[test]
    fn high_task_display() {
        let task = TaskBuilder::new("high task".to_string())
            .priority(Priority::High)
            .build();
        assert_eq!(
            format!("High Task: {}", task),
            format!("High Task: {}", "high task".red()),
        );
    }

    #[test]
    fn normal_task_display() {
        let task = TaskBuilder::new("normal task".to_string())
            .priority(Priority::Normal)
            .build();
        assert_eq!(
            format!("Normal Task: {}", task),
            format!("Normal Task: {}", "normal task".yellow()),
        );
    }

    #[test]
    fn low_task_display() {
        let task = TaskBuilder::new("low task".to_string())
            .priority(Priority::Low)
            .build();
        assert_eq!(
            format!("Low Task: {}", task),
            format!("Low Task: {}", "low task".green()),
        );
    }

    #[test]
    fn note_display() {
        let task = TaskBuilder::new("note".to_string())
            .priority(Priority::Note)
            .build();
        assert_eq!(
            format!("Note: {}", task),
            format!("Note: {}", "note".cyan()),
        );
    }

    #[test]
    fn default_display() {
        let task = TaskBuilder::new("default".to_string()).build();
        assert_eq!(
            format!("Default: {}", task),
            format!("Default: {}", "default".bold()),
        );
    }

    #[test]
    fn urgent_filter() {
        let task = TaskBuilder::new("task".to_string())
            .priority(Priority::Urgent)
            .build();
        assert_eq!(task.choose(), true);
    }

    #[test]
    fn high_filter() {
        let task = TaskBuilder::new("task".to_string())
            .priority(Priority::High)
            .build();
        assert_eq!(task.choose(), true);
    }
}
