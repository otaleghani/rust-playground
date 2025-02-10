use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Pool, Row, Sqlite, SqlitePool, TypeInfo, ValueRef};
use std::collections::HashMap;
use std::str;
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

    // Remember to activate the foreign keys
    let activate_query = "PRAGMA foreign_keys = ON";
    sqlx::query(&activate_query)
        .execute(&state.clone().master)
        .await
        .unwrap();

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

    let json_data = r#"
        {
            "id": 2,
            "name": "The new row"
        }
    "#;

    add_to_table(first, state.clone(), table, json_data)
        .await
        .unwrap();

    delete_from_row(first, state.clone(), table, "2")
        .await
        .unwrap();

    let update_json_data = r#"
        {
            "name": "The old hunter"
        }
    "#;

    update_from_row(first, state.clone(), table, update_json_data, "1")
        .await
        .unwrap();

    let table_two = "tableTwo";
    let json_data_two = r#"
        {
            "items": [
                { "type": "Number", "name": "id" },
                { "type": "String", "name": "name" }
            ]
        }
    "#;

    add_table(first, state.clone(), table_two, json_data_two)
        .await
        .unwrap();

    let content_tb_two = r#"
        {
            "id": 0,
            "name": "Anvendi NANDO"
        }  
    "#;
    // let json_data = r#"
    //     {
    //         "id": 2,
    //         "name": "The new row"
    //     }
    // "#;

    add_to_table(first, state.clone(), table_two, &content_tb_two)
        .await
        .unwrap_or_else(|err| println!("Got this error: {:?}", err));
    println!("Added into tb 2");

    create_foreign_key(first, state.clone(), table, table_two)
        .await
        .unwrap();

    let content_foreign = r#"
        {
            "tableOne_id": 0,
            "tableTwo_id": 0
        }
    "#;
    // TODO: Specific function that connects two id in a foreign table
    add_to_table(first, state.clone(), "tableOne_tableTwo", &content_foreign)
        .await
        .unwrap_or_else(|err| println!("Got this error: {:?}", err));

    visualize_references(first, state.clone(), table)
        .await
        .unwrap();

    get_references(first, state.clone(), table, 0)
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

    let query = "
    CREATE TABLE IF NOT EXISTS databases (
        id TEXT PRIMARY KEY,
        path TEXT NOT NULL)
    ";
    sqlx::query(query).execute(&pool).await?;

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
    for i in &items {
        println!("THE VALIDATED STRUCT: {:?}", i.r#type);
    }
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

        // The first it should alwayt be the primary key
        if index == 0 {
            query = format!("{} PRIMARY KEY", query);
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
    println!("The struct:  {:?}", structure);

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
            println!("{:?}", structure);
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

    println!("{:<50?} STRUCTURE OF {}", rows, table_name);

    Ok(rows)
}

// Delete row
async fn delete_from_row(
    database_name: &str,
    state: State,
    table_name: &str,
    id: &str,
) -> Result<(), sqlx::Error> {
    // It should return a specific error like "NOT_FOUND" or "UNABLE_"
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    let query = format!("DELETE FROM {} WHERE id = {}", table_name, id);
    println!("THE DELETE QUERY: {}", query);

    sqlx::query(&query).execute(&pool).await?;

    Ok(())
}

// Update row
async fn update_from_row(
    database_name: &str,
    state: State,
    table_name: &str,
    content: &str,
    id: &str,
) -> Result<(), sqlx::Error> {
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    // Validation is needed
    // Get the structure of the table that you want to add to
    let structure = get_structure(database_name, state.clone(), table_name).await?;

    let mut json_value: Value = serde_json::from_str(content).unwrap();
    let json_map = json_value.as_object_mut().unwrap();

    let mut sql_values = Vec::new();
    for (idx, val) in json_map {
        // Create the query
        sql_values.push(format!("{} = {}", idx, val));
        if !structure.contains_key(idx) {
            panic!("the key {} does not exist", idx);
        }
    }

    let sql_query = sql_values.join(", ");

    let query = format!(
        "UPDATE {} SET {} WHERE id = '{}'",
        table_name, sql_query, id
    );

    sqlx::query(&query).execute(&pool).await?;
    Ok(())
}

// TODO: Bulk update?

// Create Foreign keys
// The idea is to make a connection from two tables by creating a third table that store the connected ids.7
// By doing something like this you add a bit of overhead but you make one to one, one to many and many to many possible in one way
// One to one -> Check if the two id exists, if no create connection, else do not create connection
// One to many -> Check if one of two exist,
async fn create_foreign_key(
    database_name: &str,
    state: State,
    table_name: &str,
    other_table_name: &str,
) -> Result<(), sqlx::Error> {
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    // Connect the different ids
    let query = format!(
        "
            CREATE TABLE IF NOT EXISTS {0}_{1} (
                {0}_id NUMBER NOT NULL,
                {1}_id NUMBER NOT NULL,
                FOREIGN KEY ({0}_id) REFERENCES {0}(id) ON DELETE CASCADE,
                FOREIGN KEY ({1}_id) REFERENCES {1}(id) ON DELETE CASCADE,
                PRIMARY KEY ({0}_id, {1}_id)
            )
        ",
        table_name, other_table_name
    );

    sqlx::query(&query).execute(&pool).await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
struct Reference {
    name: String,
}

// Visualize the different foreign keys of a given table
async fn visualize_references(
    database_name: &str,
    state: State,
    table_name: &str,
) -> Result<(), sqlx::Error> {
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    // Gets the name of all the references table
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '{}_%'",
        table_name
    );
    let result: Vec<Reference> = sqlx::query_as(&query).fetch_all(&pool).await?;

    // let
    for val in result {
        println!("||> {:<12} {}", "FOREIGN KEY", val.name);
    }

    Ok(())
}

// Create a connection between two records
async fn create_reference(
    database_name: &str,
    state: State,
    table_name: &str,
    id: &str,
    other_table_name: &str,
    other_id: &str,
) -> Result<(), sqlx::Error> {
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    let query = format!(
        "INSERT OR IGNORE INTO {0}_{1} ({0}_id, {1}_id) VALUES ({2}, {3})",
        table_name, other_table_name, id, other_id
    );
    sqlx::query(&query).execute(&pool).await?;
    Ok(())
}

// Delete a conection between two records
async fn delete_reference(
    database_name: &str,
    state: State,
    table_name: &str,
    id: &str,
    other_table_name: &str,
    other_id: &str,
) -> Result<(), sqlx::Error> {
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    // let query = format!("DELETE FROM {} WHERE id = '{}'", table_name, id);
    let query = format!(
        "DELETE FROM {0}_{1} WHERE {0}_id = '{2}' AND {1}_id = '{3}'",
        table_name, other_table_name, id, other_id
    );

    sqlx::query(&query).execute(&pool).await?;
    Ok(())
}

#[derive(sqlx::FromRow, Debug)]
struct ForeignId {
    id: i32,
}

// Given an item id of a given table, return all the different refereces of said id
async fn get_references(
    database_name: &str,
    state: State,
    table_name: &str,
    id: i32,
) -> Result<(), sqlx::Error> {
    println!("");
    // Get the pool
    let pool = find_database(database_name, state.clone()).unwrap();

    // Gets the name of all the references table (e.g.: [tableOne_tableTwo, tableOne_tableTree])
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '{}_%'",
        table_name
    );
    let result: Vec<Reference> = sqlx::query_as(&query).fetch_all(&pool).await?;
    println!("||> SELECTED ALL THE FOREIGN KEYS");

    // For each val (which is the foreign key table)
    for val in result {
        // Select in the foreign key table the one that have the id
        let (_, other_table) = val.name.split_once("_").unwrap();
        let select_foreign = format!(
            "SELECT {0}_id AS id FROM {1} WHERE {2}_id = {3}",
            other_table, val.name, table_name, id
        );
        println!("{}", select_foreign);

        // Here you have every foreign id, and for each and everyone, you select the item
        let foreign_ids = sqlx::query(&select_foreign).fetch_all(&pool).await?;
        for row_id in foreign_ids {
            let idx: i32 = row_id.get("id");
            println!("||> SELECTED ALL THE FOREIGN KEYS {:?}", idx);

            let select_foreign = format!("SELECT * FROM {0} WHERE id = {1}", other_table, idx);
            let item_row = sqlx::query(&select_foreign).fetch_all(&pool).await?;

            for item in item_row {
                let name: String = item.get("name");
                println!("||> Item name: {}", name);
            }
        }

        // for ids in foreign_ids {
        //     // YOU WERE HERE -> You need to return all the different references of one item from a given table name
        //     let select_foreign_items =
        //         format!("SELECT * FROM {0} WHERE id = {1}", table_name, ids.id);

        //     let sus: Vec<ForeignId> = sqlx::query_as(&select_foreign_items)
        //         .fetch_all(&pool)
        //         .await?;

        //     // let id_json = convert_to_json(id);
        //     println!("||> {:<12} {:?}", "ID_FOREIGN", sus[0]);
        // }

        // // We want to execute this thing, but then we enter the RAW data world
        // // because we do not know what this table looks like.

        // let result = sqlx::query(&select_foreign).fetch_all(&pool).await?;
        // let json = convert_to_json(result);
        // println!("||> {:<12} {}", "FOREIGN DATA CONNECTION", json);
    }

    Ok(())
}

// From a SqliteRow return a serde_json object!
fn convert_to_json(rows: Vec<SqliteRow>) -> Value {
    let mut json_rows = Vec::with_capacity(rows.len());

    // Iterate over each row.
    for row in rows {
        let mut object = serde_json::Map::new();

        // For each column in the row...
        for col in row.columns() {
            let col_name = col.name();
            let json_value = {
                // First, try to extract the value as an i64.
                if let Ok(Some(i)) = row.try_get::<Option<i64>, _>(col_name) {
                    Value::Number(Number::from(i))
                }
                // If that fails, try as f64.
                else if let Ok(Some(f)) = row.try_get::<Option<f64>, _>(col_name) {
                    if let Some(num) = Number::from_f64(f) {
                        Value::Number(num)
                    } else {
                        Value::Null
                    }
                }
                // Next, try as a String.
                else if let Ok(Some(s)) = row.try_get::<Option<String>, _>(col_name) {
                    Value::String(s)
                }
                // Otherwise, use JSON Null.
                else {
                    Value::Null
                }
            };

            object.insert(col_name.to_string(), json_value);
        }

        json_rows.push(Value::Object(object));
    }

    Value::Array(json_rows)
}

// TODO: Create connection between two records
// async fn create_reference(database_name: &str, state: State, table_name: &str) {}

// TODO: Get foreign key connection -> given one table, return all of the present connections

// TODO: Drop foreign key (delete the connection)

// TODO: Error column -> The idea here is that whenever you change the metadata you'll also need to handle the validation of every affected column
// TODO: Metedata management ( Read, Write, Update metadata column ) -> Remember metadata is TEXT "database.table.column" that binds another TEXT which is the JSON
