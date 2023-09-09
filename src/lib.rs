use rocket::{get, serde::json::Json};

use rocket::fs::NamedFile;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub async fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).await.ok()
}

#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, JsonSchema)]
pub struct Project {
    name: String,
    description: String,
    git_url: String,
}

#[openapi]
#[get("/current-project")]
pub fn current_project() -> Json<Project> {
    Json(Project {
        name: "ChatContextProvider".to_string(),
        description: "A ChatGPT plugin to provide contextual information about GitHub projects."
            .to_string(),
        git_url: "https://github.com/chriamue/chat-context-provider.git".to_string(),
    })
}
