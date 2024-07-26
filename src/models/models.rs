use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretMessage {
    pub id: uuid::Uuid,
    pub message: String,
}

impl From<&Row> for SecretMessage {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            message: row.get("message"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSecretMessage {
    pub message: String,
}