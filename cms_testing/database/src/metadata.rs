use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnMetadata {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableMetadata {
    pub name: String,
    pub description: Option<String>,
    pub fields: Option<Vec<ColumnMetadata>>,
}
