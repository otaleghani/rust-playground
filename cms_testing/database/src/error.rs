use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    SqliteError(#[from] sqlx::Error),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Table error: {0}")]
    TableError(String),

    #[error("Unable to create file: {0}")]
    UnableToCreate(String),
}
