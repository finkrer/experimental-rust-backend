mod headers;
mod routes;
mod ssl;
mod templates;

use actix_web::{middleware, App, HttpServer};
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
    })
    .bind("0.0.0.0:8080")?
    .bind_rustls("0.0.0.0:8443", ssl::get_config())?
    .run()
    .await
}
