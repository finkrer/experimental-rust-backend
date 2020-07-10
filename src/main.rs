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
            .wrap(middleware::DefaultHeaders::new()
                .header("Strict-Transport-Security", "max-age=63072000; includeSubdomains; preload")
                .header("X-Frame-Options", "SAMEORIGIN")
                .header("X-Content-Type-Options", "nosniff")
                .header("X-XSS-Protection", "1; mode=block")
                .header("Content-Security-Policy", "script-src 'self'; object-src 'none'; img-src 'self' data:; style-src 'self' https://fonts.googleapis.com; base-uri 'none'; form-action 'none'; frame-ancestors 'self'; require-trusted-types-for 'script';")
                .header("Referrer-Policy", "no-referrer")
                .header("Feature-Policy", "vibrate 'self'"))
            .wrap(RedirectSchemeBuilder::new().build())
            .wrap(middleware::NormalizePath)
            .configure(routes::config)
    })
    .bind("0.0.0.0:8080")?
    .bind_rustls("0.0.0.0:8443", ssl::get_config())?
    .run()
    .await
}
