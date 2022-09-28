mod config;
mod model;
mod api;

use actix_web::{HttpServer, App, middleware::Logger, web, Responder};
use api::tracks::{get_track};
use config::db::get_mongo_client;
use dotenv;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
// intialize_env vars
    dotenv::dotenv().ok();

    // setting logger
   std::env::set_var("RUST_LOG", "actix_web=debug");
   std::env::set_var("RUST_BACKTRACE", "1");
   env_logger::init();

    // let client = get_mongo_client()
    //     .await;
    let uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
           .app_data(web::Data::new(client.clone()))
            .wrap(logger)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
           .service(get_track)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}


async fn hello() -> impl Responder {
    format!("Hello fellow Rustacean!")
}
