use either::Either;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::path::PathBuf;

pub fn canonicalize_file<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    fs::canonicalize(path).and_then(|path| {
        if path.is_file() {
            Ok(path)
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Not a file"))
        }
    })
}

pub fn find_file(path: &str) -> Either<PathBuf, PathBuf> {
    let mut found = true;
    let path = format!("./static/{}", path);
    let path = canonicalize_file(&path)
        .or_else(|_| canonicalize_file(format!("{}.html", &path)))
        .or_else(|_| canonicalize_file(format!("{}/index.html", &path)))
        .or_else(|_| {
            found = false;
            canonicalize_file("./static/404.html")
        })
        .expect("404 page not found");
    if found {
        Either::Right(path)
    } else {
        Either::Left(path)
    }
}
