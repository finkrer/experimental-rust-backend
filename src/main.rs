use actix_files as fs;
use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use yarte::Template;

trait FromTemplate {
    fn from_template(t: impl Template) -> Self;
}

impl FromTemplate for HttpResponse {
    fn from_template(t: impl Template) -> Self {
        match t.call() {
            Err(error) => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!(
                    "{}: {}",
                    "Internal template error",
                    error.to_string()
                )),
            Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    #[derive(Template)]
    #[template(path = "index")]
    struct IndexTemplate;
    HttpResponse::from_template(IndexTemplate {})
}

#[get("/cat-photos/")]
async fn cat_photos() -> impl Responder {
    #[derive(Template)]
    #[template(path = "cat-photos/index")]
    struct CatPhotosTemplate;
    HttpResponse::from_template(CatPhotosTemplate {})
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("/etc/letsencrypt/live/finkrer.wtf/privkey.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("/etc/letsencrypt/live/finkrer.wtf/fullchain.pem").unwrap();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath)
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new()
                .header("Strict-Transport-Security", "max-age=63072000; includeSubdomains; preload")
                .header("X-Frame-Options", "SAMEORIGIN")
                .header("X-Content-Type-Options", "nosniff")
                .header("X-XSS-Protection", "1; mode=block")
                .header("Content-Security-Policy", "script-src 'self'; object-src 'none'; img-src 'self' data:; style-src 'self' https://fonts.googleapis.com; base-uri 'none'; form-action 'none'; frame-ancestors 'self'; require-trusted-types-for 'script';")
                .header("Referrer-Policy", "no-referrer")
                .header("Feature-Policy", "vibrate 'self'"))
            .service(index)
            .service(cat_photos)
            .service(fs::Files::new("/s/", "/usr/src/actix/static"))
    })
    .bind("127.0.0.1:8080")?
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
