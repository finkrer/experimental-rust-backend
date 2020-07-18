mod files;
mod routes;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
            .default_service(web::route().to(routes::error))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
