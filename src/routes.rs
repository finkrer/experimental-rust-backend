use crate::templates::FromTemplate;

use actix_files::NamedFile;
use actix_web::{get, HttpResponse, Responder, Result};
use yarte::Template;

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