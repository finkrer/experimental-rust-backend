use crate::files::find_file;
use actix_files::NamedFile;
use actix_web::{get, http, web, HttpRequest, HttpResponse, Responder, Result};
use either::Right;
use std::fs::File;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/{path:.*}")]
async fn get(req: HttpRequest, path: web::Path<String>) -> Result<impl Responder> {
    let path = find_file(&path.to_string());
    let status_code = match path {
        Right(_) => http::StatusCode::OK,
        _ => http::StatusCode::NOT_FOUND,
    };
    println!("{:?}", path);
    let result = File::open(&path)
        .and_then(|file| NamedFile::from_file(file, &path))?
        .set_status_code(status_code);
    let response = result.into_response(&req)?;
    Ok(response)
}

pub async fn error(req: HttpRequest) -> impl Responder {
    let res = format!("Method not allowed: {}", req.method());
    HttpResponse::MethodNotAllowed().body(res)
}
