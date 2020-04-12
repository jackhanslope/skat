#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../static/index.html")).ok()
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("../static"))
        .mount("/", routes![index])
        .launch();
}
