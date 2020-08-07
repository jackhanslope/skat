#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::NamedFile;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use skat::game::Round;
use std::path::Path;

#[get("/new_round")]
fn new_round() -> Json<Round> {
    let round = skat::game::new_round();
    // TODO can't actually return the round here. create a unique identifier for the round then use
    // that to store it server side. Then return identifier
    return Json(round);
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/index.html")).ok()
}

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("./static/favicon.ico")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("./static"))
        .mount("/api", routes![new_round])
        .mount("/", routes![index, favicon])
        .launch();
}
