use actix_web::{get, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use uuid::Uuid;
use crate::db::{insert_secret_message, select_secret_message};

#[post("/actix/secret")]
pub async fn post_secret_message(
    db_pool: web::Data<Pool>,
    new_secret_message: web::Json<crate::models::NewSecretMessage>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let secret_message = insert_secret_message(&client, &new_secret_message.message).await;
    return Ok(HttpResponse::Created()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string(&secret_message).unwrap()));
}

#[get("/actix/secret/{id}")]
pub async fn get_secret_message(
    db_pool: web::Data<Pool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let client: Client = db_pool.get().await.unwrap();
    let secret_message = select_secret_message(&client, id).await;
    return Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(serde_json::to_string(&secret_message).unwrap()));
}
