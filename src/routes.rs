use crate::templates::FromTemplate;

use actix_files::{Files, NamedFile};
use actix_web::{web, get, http::StatusCode, HttpResponse, HttpRequest, Responder, Result};
use yarte::Template;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(index)
    .service(cat_photos)
    .service(robots)
    .service(Files::new("/s/", "/usr/src/actix/static"));
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

#[get("/robots.txt")]
async fn robots() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/robots.txt")?)
}

pub async fn error(req: HttpRequest) -> impl Responder {
    #[derive(Template)]
    #[template(path = "404")]
    struct ErrorTemplate {
        path: String
    }
    HttpResponse::from_template_with_code(ErrorTemplate {path: req.path().to_string()}, StatusCode::NOT_FOUND)
}