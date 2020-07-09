use std::fs::File;
use std::io::BufReader;

use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

pub fn get_config() -> ServerConfig {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/finkrer.wtf/fullchain.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("/etc/letsencrypt/live/finkrer.wtf/privkey.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    config
}