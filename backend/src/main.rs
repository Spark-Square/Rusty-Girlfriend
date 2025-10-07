#[macro_use] extern crate rocket;
mod chat_response;
mod serve_frontend;
mod db_functions;
mod types;

// Do it            this ---------- way,  it avoids routing collisions that otherwise I don't know how to handle
use serve_frontend::{app, index, icons};

//LAUNCH ROCKET
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![app,
                        index,
                        icons,
                        chat_response::chat_response])
}
