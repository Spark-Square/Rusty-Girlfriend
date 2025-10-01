mod chat_response;

// ----------------- Imports ----------------------------
#[macro_use] extern crate rocket;

use rocket::{fs::NamedFile, get};
use std::path::Path;


// Do it this way, it avoids routing collisions that otherwise I don't know how to handle
#[get("/Icons/<file>")]
async fn icons(file: &str) -> Option<NamedFile> {
    // "../Icons" points to the Icons folder outside the backend
    NamedFile::open(Path::new("../Icons").join(file)).await.ok()
}
#[get("/<file>")]
async fn app(file: &str) -> Option<NamedFile> {
    // "../Icons" points to the Icons folder outside the backend
    NamedFile::open(Path::new("../frontend/dist").join(file)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("../frontend/dist/index.html").await.ok()
}
// ----------------- LAUNCH ROCKET -----------------
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![app,
                        index,
                        icons,
                        chat_response::chat_response])
}
