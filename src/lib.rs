use rocket::get;

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub async fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).await.ok()
}


#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}
