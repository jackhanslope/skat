#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::NamedFile;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::path::Path;

#[get("/new_deck")]
fn new_deck() -> Json<Vec<skat::Card>> {
    println!("Hello, world!");
    Json(skat::new_deck())
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../static/index.html")).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("../static/favicon.ico")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("../static"))
        .mount("/api", routes![new_deck])
        .mount("/", routes![index, favicon])
        .launch();
}
