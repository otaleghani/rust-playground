use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Column, Pool, Row, Sqlite, SqlitePool, TypeInfo, ValueRef};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static MASTER: &str = "master.db";

#[derive(Clone)]
struct Database {
    pool: Pool<Sqlite>,
}

#[derive(Clone)]
struct State {
    projects: Arc<RwLock<HashMap<String, Database>>>,
    master: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let master_pool = match create_master_database().await {
        Ok(some) => some,
        Err(err) => panic!("Error creating the master database: {:?}", err),
    };

    let state = State {
        master: master_pool,
        projects: Arc::new(RwLock::new(HashMap::new())),
    };

    let first = "first.db";
    create_database(first, state.clone())
        .await
        .unwrap_or_else(|err| panic!("Error creating {}, encountered error: {:?}", first, err));

    let second = "second.db";
    create_database(second, state.clone())
        .await
        .unwrap_or_else(|err| panic!("Error creating {}, encountered error: {:?}", second, err));

    let json_data = r#"
        {
            "items": [
                { "type": "Number", "name": "id" },
                { "type": "String", "name": "name" }
            ]
        }
    "#;

    let table = "tableOne";
    add_table(first, state.clone(), table, json_data)
        .await
        .unwrap();

    // get_structure(first, state.clone(), table).await.unwrap();
    let json_data = r#"
        {
            "id": 1,
            "name": "testing"
        }
    "#;

    add_to_table(first, state.clone(), table, json_data)
        .await
        .unwrap();
}

// Opens or creates a database connection from a name
async fn create_database(filename: &str, state: State) -> Result<(), sqlx::Error> {
    println!("||> {:<12} = create_database - {}", "CREATE", filename);
    // Opens the file, creating it if it doesn't exists
    tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(filename)
        .await
        .unwrap();

    // Opens the connection
    let pool = SqlitePool::connect(&filename).await?;

    // Adds the database data inside the master database
    sqlx::query("INSERT OR IGNORE INTO databases (id, path) VALUES (?, ?)")
        .bind(filename)
        .bind(filename)
        .execute(&state.master)
        .await?;

    // Insert newly created pool in the state
    state
        .projects
        .write()
        .unwrap()
        .insert(filename.to_string(), Database { pool });

    Ok(())
}

// Opens or creates the database connection with the master database
async fn create_master_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    println!("||> {:<12} = create_master_database - {}", "CREATE", MASTER);
    // Opens the file, creating it if it doesn't exists
    tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(MASTER)
        .await
        .unwrap();

    // Opens the connection
    let pool = SqlitePool::connect(MASTER).await?;

    Ok(pool)
}

// Returns the database connection of the db with the given name
fn find_database(name: &str, state: State) -> Result<Pool<Sqlite>, sqlx::Error> {
    println!("||> {:<12} = find_database - {}", "FIND", name);
    let pool = state
        .projects
        .read()
        .unwrap()
        .get(name)
        .cloned()
        .unwrap()
        .pool;

    Ok(pool)
}

#[derive(Debug, Clone, Serialize)]
enum Error {
    DeserializeFail,
}
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ColumnStructure {
    name: String,
    r#type: AcceptedTypes,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum AcceptedTypes {
    Bool,
    Number,
    String,
    Array,
    Object,
}

// Validates the given fields from a JSON
async fn validate_fields(json: &str) -> Result<Vec<ColumnStructure>, Error> {
    let value: Value = serde_json::from_str(json).unwrap();
    let items: Vec<ColumnStructure> = serde_json::from_value(value["items"].clone()).unwrap();
    Ok(items)
}

// Add a new table with the given name to the database
async fn add_table(
    database_name: &str,
    state: State,
    table_name: &str,
    fields: &str,
) -> Result<(), sqlx::Error> {
    println!("||> {:<12} = add_table - {}", "ADD", table_name);
    let pool = find_database(database_name, state).unwrap();
    let parsed_fields = validate_fields(fields).await.unwrap();

    let mut query = String::from(format!("CREATE TABLE IF NOT EXISTS {} (", table_name));
    for (index, field) in parsed_fields.iter().enumerate() {
        match field.r#type {
            AcceptedTypes::Number => query = format!("{} {} INTEGER", query, field.name),
            AcceptedTypes::String => query = format!("{} {} TEXT", query, field.name),
            _ => {}
        }

        if index != parsed_fields.len() - 1 {
            query = format!("{},", query);
        }
    }

    query = format!("{})", query);
    println!("{}", query);

    // Write the query
    sqlx::query(&query).execute(&pool).await?;

    Ok(())
}

// Add a value inside dynamicly
async fn add_to_table(
    database_name: &str,
    state: State,
    table_name: &str,
    content: &str,
) -> Result<(), sqlx::Error> {
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    // Get the structure of the table that you want to add to
    let structure = get_structure(database_name, state.clone(), table_name).await?;

    let mut value: Value = serde_json::from_str(content).unwrap();
    let val = value.as_object_mut().unwrap();
    // let valuess = val.take();

    // Validate the data with the given structure
    let mut keys = String::new();
    let mut values = String::new();
    let mut first = true;
    for (idx, val) in val {
        // Create the query
        if first {
            keys = String::from(format!("{}", idx));
            values = String::from(format!("{}", val.take()));
            first = false;
        } else {
            keys = String::from(format!("{}, {}", keys, idx));
            values = String::from(format!("{}, {}", values, val.take()));
        }
        if !structure.contains_key(idx) {
            panic!("the key {} does not exist", idx);
        }
    }
    let query = format!("INSERT INTO {} ({}) VALUES ({})", table_name, keys, values);

    // Execute the query
    sqlx::query(&query).execute(&pool).await?;

    Ok(())
}

// Get the structure of the data
async fn get_structure(
    database_name: &str,
    state: State,
    table_name: &str,
) -> Result<HashMap<String, String>, sqlx::Error> {
    let pool = find_database(database_name, state).unwrap();
    let query = format!("PRAGMA table_info({})", table_name);

    let result = sqlx::query(&query).fetch_all(&pool).await?;
    let mut rows = HashMap::new();
    for row in result {
        let mut row_map = HashMap::new();
        let name: String = row.get("name");
        let field_type: String = row.get("type");

        rows.insert(name, field_type);

        // This is just a test to map every single raw data from sqlite to serde_json
        for column in row.columns() {
            let col_name = column.name();
            let value = row.try_get_raw(col_name).unwrap();
            let json_value = match value.type_info().name() {
                "TEXT" => row
                    .try_get::<String, _>(col_name)
                    .map(Value::String)
                    .unwrap_or(Value::Null),
                "INTEGER" => row
                    .try_get::<i64, _>(col_name)
                    .map(|v| Value::Number(v.into()))
                    .unwrap_or(Value::Null),
                "REAL" => row
                    .try_get::<f64, _>(col_name)
                    .map(|v| serde_json::Number::from_f64(v))
                    .unwrap()
                    .map(Value::Number)
                    .unwrap_or(Value::Null),
                "BOOLEAN" => row
                    .try_get::<bool, _>(col_name)
                    .map(Value::Bool)
                    .unwrap_or(Value::Null),
                _ => Value::Null,
            };
            row_map.insert(col_name, json_value);
        }
    }

    println!("{:?}", rows);

    Ok(rows)
}
