// error.rs
//
// define an error type

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustaskError {
    #[error("Failed to perform IO")]
    IOError(#[from] std::io::Error),

    #[error("Cannot find task {}", .0)]
    OutOfBounds(usize),

    #[error("Project {} not found", .0)]
    ProjectNotFound(String),

    #[error("Project {} already exists", .0)]
    ProjectNameTaken(String),

    #[allow(unused)]
    #[error("Task file `{}` not found", .0)]
    TaskFileNotFound(String),

    #[error("Failed to serialize")]
    SerializationError(#[from] serde_json::Error),
}

impl std::cmp::PartialEq for RustaskError {
    fn eq(&self, other: &RustaskError) -> bool {
        match self {
            RustaskError::IOError(_a) => match other {
                RustaskError::IOError(_b) => true,
                _ => false,
            },
            RustaskError::OutOfBounds(a) => match other {
                RustaskError::OutOfBounds(b) => a == b,
                _ => false,
            },
            RustaskError::TaskFileNotFound(a) => match other {
                RustaskError::TaskFileNotFound(b) => a == b,
                _ => false,
            },
            RustaskError::ProjectNameTaken(a) => match other {
                RustaskError::ProjectNameTaken(b) => a == b,
                _ => false,
            },
            RustaskError::ProjectNotFound(a) => match other {
                RustaskError::ProjectNotFound(b) => a == b,
                _ => false,
            },
            RustaskError::SerializationError(_a) => match other {
                RustaskError::SerializationError(_b) => true,
                _ => false,
            },
        }
    }
}
