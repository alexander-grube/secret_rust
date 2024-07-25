mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;
    use tokio_postgres::Row;

    #[derive(Debug, Serialize, Deserialize, PostgresMapper)]
    #[pg_mapper(table = "secret_message")]
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
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;
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
            .map(|row| SecretMessage::from_row_ref(row).unwrap())
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
            .map(|row| SecretMessage::from_row_ref(row).unwrap())
            .collect::<Vec<SecretMessage>>()
            .pop()
            .unwrap();
    }
}

mod handlers {
    use actix_web::{get, post, web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};
    use uuid::Uuid;

    use crate::db;

    #[post("/actix/secret")]
    pub async fn post_secret_message(
        db_pool: web::Data<Pool>,
        new_secret_message: web::Json<crate::models::NewSecretMessage>
    ) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.unwrap();
        let secret_message = db::insert_secret_message(&client, &new_secret_message.message).await;
        return Ok(HttpResponse::Created()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&secret_message).unwrap()));
    }

    #[get("/actix/secret/{id}")]
    pub async fn get_secret_message(
        db_pool: web::Data<Pool>,
        path: web::Path<Uuid>
    ) -> Result<HttpResponse, Error> {
        let id = path.into_inner();
        let client: Client = db_pool.get().await.unwrap();
        let secret_message = db::select_secret_message(&client, id).await;
        return Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&secret_message).unwrap()));
    }
}

use ::config::Config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use handlers::{
    get_secret_message,
    post_secret_message,
};
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(post_secret_message)
            .service(get_secret_message)
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}