// error.rs
//
// define an error type

use serde_json;
use std::fmt;
#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    OutOfBounds(usize),
    ProjectNotFound(String),
    ProjectNameTaken(String),
    TaskFileNotFound,
    SerializationError,
}

impl std::error::Error for self::Error {}

impl std::cmp::PartialEq for self::Error {
    fn eq(&self, other: &self::Error) -> bool {
        match self {
            Error::IOError(_err) => match other {
                Error::IOError(_a) => true,
                _ => false,
            },
            Error::OutOfBounds(a) => match other {
                Error::OutOfBounds(b) => a == b,
                _ => false,
            },
            Error::TaskFileNotFound => match other {
                Error::TaskFileNotFound => true,
                _ => false,
            },
            Error::ProjectNameTaken(a) => match other {
                Error::ProjectNameTaken(b) => a == b,
                _ => false,
            },
            Error::ProjectNotFound(a) => match other {
                Error::ProjectNotFound(b) => a == b,
                _ => false,
            },
            Error::SerializationError => match other {
                Error::SerializationError => true,
                _ => false,
            },
        }
    }
}

impl fmt::Display for self::Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(_) => self.fmt(f),
            Error::OutOfBounds(idx) => write!(f, "Cannot find task {}", idx),
            Error::TaskFileNotFound => write!(f, "Task file not found"),
            Error::ProjectNotFound(name) => write!(f, "Project {} not found", name),
            Error::ProjectNameTaken(name) => write!(f, "Project {} already exists", name),
            Error::SerializationError => write!(f, "Serialization Error"),
        }
    }
}

impl From<std::io::Error> for self::Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => self::Error::TaskFileNotFound,
            _ => self::Error::IOError(error),
        }
    }
}

impl From<serde_json::Error> for self::Error {
    fn from(_: serde_json::Error) -> Self {
        self::Error::SerializationError
    }
}
