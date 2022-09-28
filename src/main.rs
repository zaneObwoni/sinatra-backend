mod config;

use actix_web::{HttpServer, App, middleware::Logger, web, Responder};
use config::db::get_mongo_client;
use dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
// intialize_env vars
    dotenv::dotenv().ok();

    // setting logger
   std::env::set_var("RUST_LOG", "actix_web=debug");
   std::env::set_var("RUST_BACKTRACE", "1");
   env_logger::init();

    let client = get_mongo_client().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            // link app to mongo db
            .app_data(client.clone())
            .wrap(logger)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}


async fn hello() -> impl Responder {
    format!("Hello fellow Rustacean!")
}
