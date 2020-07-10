use actix_web::middleware::DefaultHeaders;

pub fn get_default() -> DefaultHeaders {
    DefaultHeaders::new()
    .header("Strict-Transport-Security", "max-age=63072000; includeSubdomains; preload")
    .header("X-Frame-Options", "SAMEORIGIN")
    .header("X-Content-Type-Options", "nosniff")
    .header("X-XSS-Protection", "1; mode=block")
    .header("Content-Security-Policy", "script-src 'self'; object-src 'none'; img-src 'self' data:; style-src 'self' https://fonts.googleapis.com; base-uri 'none'; form-action 'none'; frame-ancestors 'self'; require-trusted-types-for 'script';")
    .header("Referrer-Policy", "no-referrer")
    .header("Feature-Policy", "vibrate 'self'")
}