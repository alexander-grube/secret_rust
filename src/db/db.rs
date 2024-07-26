use deadpool_postgres::Client;
use uuid::Uuid;
use crate::models::SecretMessage;

pub async fn insert_secret_message(client: &Client, message: &str) -> SecretMessage {
    let _stmt = "INSERT INTO secret_message (message) VALUES ($1) RETURNING *";
    let stmt = client.prepare(&_stmt).await.unwrap();

    return client
        .query(&stmt, &[&message])
        .await
        .unwrap()
        .iter()
        .map(|row| SecretMessage::from(row))
        .collect::<Vec<SecretMessage>>()
        .pop()
        .unwrap();
}

pub async fn select_secret_message(client: &Client, id: Uuid) -> SecretMessage {
    let _stmt = "SELECT * FROM secret_message WHERE id = $1";
    let stmt = client.prepare(&_stmt).await.unwrap();

    return client
        .query(&stmt, &[&id])
        .await
        .unwrap()
        .iter()
        .map(|row| SecretMessage::from(row))
        .collect::<Vec<SecretMessage>>()
        .pop()
        .unwrap();
}
