mod headers;
mod routes;
mod ssl;
mod templates;

use actix_web::{web, middleware, App, HttpServer};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(headers::get_default())
            .wrap(RedirectSchemeBuilder::new().build())
            .wrap(middleware::NormalizePath)
            .configure(routes::config)
            .default_service(web::route().to(routes::error))
    })
    .bind("0.0.0.0:8080")?
    .bind_openssl("0.0.0.0:8443", ssl::get_builder())?
    .run()
    .await
}
