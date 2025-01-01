// src/todo/todo_errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoServiceError {
    #[error("Invalid todo ID")]
    InvalidId,
    #[error("Todo not found")]
    NotFound,
    #[error("Insertion failed")]
    InsertionFailed,
    #[error("Database error: {0}")]
    DatabaseError(String),
}
