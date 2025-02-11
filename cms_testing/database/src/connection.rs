use crate::error::DatabaseError;
use sqlx::{Pool, Sqlite, SqlitePool};

static MASTER: &str = "master.db";

pub async fn open_project_connection(db_path: &str) -> Result<Pool<Sqlite>, DatabaseError> {
    tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(db_path)
        .await
        .map_err(|err| DatabaseError::UnableToCreate(err.to_string()))?;

    let db = SqlitePool::connect(db_path)
        .await
        .map_err(|err| DatabaseError::SqliteError(err))?;

    // TODO: Initialize database with metadata

    Ok(db)
}

pub async fn open_master_connection() -> Result<Pool<Sqlite>, DatabaseError> {
    tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(MASTER)
        .await
        .map_err(|err| DatabaseError::UnableToCreate(err.to_string()))?;

    let db = SqlitePool::connect(MASTER)
        .await
        .map_err(|err| DatabaseError::SqliteError(err))?;

    // TODO: Inizialize metadata if no exists

    Ok(db)
}

// TODO: I need a way to return all the connections in the masterdb
