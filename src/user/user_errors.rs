// src/user/user_errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("Invalid user ID")]
    InvalidId,
    #[error("User not found")]
    NotFound,
    #[error("Insertion failed")]
    InsertionFailed,
    #[error("Database error: {0}")]
    DatabaseError(String),
}
