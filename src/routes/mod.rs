mod heme;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

pub use self::heme::*;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("html/index.html")
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("html/").join(file)).ok()
}
