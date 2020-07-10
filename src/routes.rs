use crate::templates::FromTemplate;

use actix_files::{Files, NamedFile};
use actix_web::{web, get, HttpResponse, Responder, Result};
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