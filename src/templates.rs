use actix_web::{http::StatusCode, HttpResponse};
use yarte::Template;

pub trait FromTemplate {
    fn from_template(t: impl Template) -> Self;
    fn from_template_with_code(t: impl Template, code: StatusCode) -> Self;
}

impl FromTemplate for HttpResponse {
    fn from_template(t: impl Template) -> Self {
        Self::from_template_with_code(t, StatusCode::OK)
    }

    fn from_template_with_code(t: impl Template, code: StatusCode) -> Self {
        match t.call() {
            Err(error) => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!(
                    "{}: {}",
                    "Internal template error",
                    error.to_string()
                )),
            Ok(content) => HttpResponse::build(code).content_type("text/html").body(content),
        }
    }
}