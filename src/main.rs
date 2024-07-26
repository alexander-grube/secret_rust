mod handlers;
mod db;
mod config;
mod models;

use ::config::Config;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use handlers::{get_secret_message, post_secret_message};
use config::ExampleConfig;

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