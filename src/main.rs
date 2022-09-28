use actix_web::{HttpServer, App, middleware::Logger, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "debug");
   std::env::set_var("RUST_BACKTRACE", "1");
   env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
