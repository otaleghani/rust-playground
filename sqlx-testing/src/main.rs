use sqlx::{Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static MASTER: &str = "master.db";

type TableSchema = Vec<Field>;
type DatabaseSchema = HashMap<String, TableSchema>;

struct Database {
    pool: Pool<Sqlite>,
    schema: Arc<RwLock<DatabaseSchema>>,
}

// type Connections = Arc<RwLock<HashMap<String, Pool<Sqlite>>>>;

#[derive(Clone)]
struct State {
    projects: Arc<RwLock<HashMap<String, Database>>>,
    master: Pool<Sqlite>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Hashmap to search the correct db

    // Open the master database
    let master_pool = open_master_database().await.unwrap();
    let state = State {
        projects: Arc::new(RwLock::new(HashMap::new())),
        master: master_pool,
    };

    let first_db = "first.db";
    let pool = create_database(first_db, state.clone()).await?;
    state.projects.write().unwrap().insert(
        first_db.to_string(),
        Database {
            pool,
            schema: Arc::new(RwLock::new(HashMap::new())),
        },
    );

    let second_db = "second.db";
    let pool = create_database(second_db, state.clone()).await?;
    state.projects.write().unwrap().insert(
        first_db.to_string(),
        Database {
            pool,
            schema: Arc::new(RwLock::new(HashMap::new())),
        },
    );

    let fields = vec![
        Field {
            name: "first_table".to_string(),
            field_type: FieldType::String,
        },
        Field {
            name: "second_table".to_string(),
            field_type: FieldType::Int,
        },
    ];

    add_table(state.clone(), "first.db", fields, "new_table").await?;

    Ok(())
}

// Create a new database based on name
async fn create_database(filename: &str, state: State) -> Result<Pool<Sqlite>, sqlx::Error> {
    tokio::fs::File::create(&filename).await.unwrap();
    let pool = SqlitePool::connect(&filename).await?;

    // Inserts the database filename inside of the master database
    let query = "INSERT INTO databases (id, path) VALUES (?, ?)";

    let master_pool = state.master;

    sqlx::query(query)
        .bind(filename)
        .bind(filename)
        .execute(&master_pool)
        .await?;

    Ok(pool)
}

async fn open_master_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    tokio::fs::File::create(&MASTER).await.unwrap();
    let db = SqlitePool::connect(&MASTER).await?;
    let query = "
    CREATE TABLE IF NOT EXISTS databases (
        id TEXT PRIMARY KEY,
        path TEXT NOT NULL)
    ";
    sqlx::query(query).execute(&db).await?;
    Ok(db)
}

// What are the op that you want to do?
// Add a new table
// Remove a table
// Update a table

#[derive(Clone)]
pub enum FieldType {
    String,
    Int,
}

#[derive(Clone)]
struct Field {
    name: String,
    field_type: FieldType,
    // Here you'll have like PRIMARY KEY, FOREIGN KEYs ETC
}

async fn add_table(
    state: State,
    database: &str,
    fields: Vec<Field>,
    name: &str,
) -> Result<(), sqlx::Error> {
    // Creates the query
    let mut query = String::from(format!("CREATE TABLE IF NOT EXISTS {} (", name));
    for (index, field) in fields.iter().enumerate() {
        match field.field_type {
            FieldType::String => query = format!("{} {} TEXT", query, field.name),
            FieldType::Int => query = format!("{} {} INTEGER", query, field.name),
        }
        if index != fields.len() - 1 {
            query = format!("{},", query);
        }
    }

    query = format!("{})", query);
    println!("{}", query);

    let projects = state.projects.write().unwrap();
    let project_database = projects.get(&database.to_string()).unwrap();

    sqlx::query(&query).execute(&project_database.pool).await?;

    let mut project_schema = project_database.schema.write().unwrap();
    project_schema.insert(name.to_string(), fields);

    Ok(())
}

struct Datab {
    id: String,
    path: String,
}
async fn load_databases(state: State) -> Result<(), sqlx::Error> {
    // TODO: Check if there is a master database
    let master_pool = state.master;
    let database_info_query = "PRAGMA table_list";
    let table_info_query = "PRAGMA table_info('{}')";

    let databases_query = "SELECT * FROM databases";

    // let result = sqlx::query(databases_query).execute(&master_pool).await?;
    let result = sqlx::query_as!(Datab, "SELECT * FROM databases")
        .fetch_all(&master_pool)
        .await?;

    // TODO: Check databases in master database
    // TODO: Foreach database, get the different tables
    // TODO: Foreach table, get the different columns

    Ok(())
}
