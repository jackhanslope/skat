#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]

use rocket::response::NamedFile;
use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;
use std::path::Path;

#[get("/new_deck")]
fn new_deck() -> Vec<skat::Card> {
    println!("Hello, world!");
    //...
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../static/index.html")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("../static"))
        .mount("/api", routes![new_deck])
        .mount("/", routes![index])
        .launch();
}
