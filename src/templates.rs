use actix_web::HttpResponse;
use yarte::Template;

pub trait FromTemplate {
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