use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static MASTER: &str = "master.db";

type TableSchema = Vec<Field>;
type DatabaseSchema = HashMap<String, TableSchema>;

struct Database {
    pool: Pool<Sqlite>,
    schema: Arc<RwLock<DatabaseSchema>>,
}

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
        second_db.to_string(),
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

    add_table(state.clone(), "second.db", fields.clone(), "new_table").await?;
    add_table(state.clone(), "first.db", fields.clone(), "new_table").await?;
    load_databases(state.clone()).await?;

    Ok(())
}

// Create a new database based on name
async fn create_database(filename: &str, state: State) -> Result<Pool<Sqlite>, sqlx::Error> {
    let _ = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Appends instead of truncating
        .open(filename)
        .await
        .unwrap();
    // tokio::fs::File::create(&filename).await.unwrap();
    let pool = SqlitePool::connect(&filename).await?;

    // Inserts the database filename inside of the master database
    let query = "INSERT OR IGNORE INTO databases (id, path) VALUES (?, ?)";

    let master_pool = state.master;

    sqlx::query(query)
        .bind(filename)
        .bind(filename)
        .execute(&master_pool)
        .await?;

    Ok(pool)
}

async fn open_master_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let _ = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true) // Appends instead of truncating
        .open(&MASTER)
        .await
        .unwrap();
    // tokio::fs::File::create(&MASTER).await.unwrap(); // This one will truncate the content to 0 lenght,
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
    Sus,
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
            FieldType::Sus => query = format!("{} {} TEXT", query, field.name),
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

#[derive(sqlx::FromRow)]
struct Datab {
    id: String,
    path: String,
}

#[derive(sqlx::FromRow)]
struct Db {
    name: String,
}

#[derive(Debug, sqlx::FromRow)]
struct Table {
    name: String,
    r#type: String,
}

async fn load_databases(state: State) -> Result<(), sqlx::Error> {
    // Check if there is a master database
    println!("==> Starting loading...");
    let master_pool = state.master.clone();

    // Prepare the queries
    let databases_query = "SELECT * FROM databases";

    // Check databases in master database
    let databases = sqlx::query_as::<_, Datab>(databases_query)
        .fetch_all(&master_pool)
        .await?;

    // Foreach database, get the different tables
    for db in &databases {
        println!("==> Database {}", db.id);

        // Create pools
        let pool = create_database(&db.path, state.clone()).await?;
        state.projects.write().unwrap().insert(
            db.path.to_string(),
            Database {
                pool: pool.clone(),
                schema: Arc::new(RwLock::new(HashMap::new())),
            },
        );
        println!("==> Database pool opened");

        // Get table names
        let tables =
            sqlx::query_as::<_, Db>("SELECT name FROM pragma_table_list WHERE type = 'table'")
                .fetch_all(&pool)
                .await?;

        for tb in &tables {
            println!("==> Table {:>4}", tb.name);

            // Foreach table, get the different columns
            let table_info_query =
                format!("SELECT name, type FROM pragma_table_info('{}')", tb.name);
            let t = sqlx::query_as::<_, Table>(&table_info_query)
                .fetch_all(&pool)
                .await?;

            println!("{:?}", t);

            // Create the correct HashMap to store the different columns inside
            for values in &t {
                println!("==> Column {:8}", values.name);

                let data = state.projects.write().unwrap();
                let databasa = data.get(&db.path.to_string()).unwrap();
                let mut schema = databasa.schema.write().unwrap();
                schema.insert(
                    values.name.clone(),
                    vec![Field {
                        name: values.name.clone(),
                        field_type: match values.r#type.as_str() {
                            "TEXT" => FieldType::String,
                            "INTEGER" => FieldType::Int,
                            _ => FieldType::Sus,
                        },
                    }],
                );
            }
        }
        println!("");
    }

    Ok(())
}

// TODO: Add a dummy data to a table using JSON
async fn print_to_db(
    database: String,
    table: String,
    state: State,
    data: HashMap<String, serde_json::Value>,
) -> Result<(), sqlx::Error> {
    // The table structure should already be in the state
    // Get database
    let projects = state.projects.read().unwrap();
    let project_database = projects.get(&database.to_string()).unwrap();
    let pool = project_database.pool.clone();

    let schema = sqlx::query("PRAGMA table_info(users)")
        .fetch_all(&pool)
        .await?;

    for row in &schema {
        let sus: i32 = row.get("sandro");
    }

    // let valid_columns: Vec<String> = schema.iter().map(|row| row.get("sus")).collect();

    // for key in data.keys() {
    //     if !valid_columns.contains(key) {
    //         eprintln!("Invalid column: {}", key);
    //         return Ok(()); // Exit early if an invalid column is found
    //     }
    // }

    // Now we can run the SQL -> We kinda want to understand if said
    //

    // TODO: Get structure

    // TODO: Build query

    Ok(())
}
// TODO: Read data from a table by a string
async fn read_from_db(database: String, table: String, state: State) -> Result<(), sqlx::Error> {
    Ok(())
}
