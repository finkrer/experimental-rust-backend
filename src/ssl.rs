use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub fn get_builder() -> openssl::ssl::SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("/etc/letsencrypt/live/finkrer.wtf/privkey.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("/etc/letsencrypt/live/finkrer.wtf/fullchain.pem").unwrap();
    builder
}