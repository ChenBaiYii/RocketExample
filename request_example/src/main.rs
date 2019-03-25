#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::{PathBuf, Path};
use rocket::response::NamedFile;


#[get("/users/<name>/<age>/<cool>")]
fn handler(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("you are cool {} year old, {}!", age, name)
    } else {
        format!("{}, we are talk about your coolness.", name)
    }
}

//#[get("/page/<path..>")]
//fn get_page(path: PathBuf) -> T {
//    "..."
//}

//#[get("/<file..>")]
//fn files(path: PathBuf) -> Option<NamedFile> {
//    NamedFile::open(Path::new("static/").join(file)).ok()
//}


#[get("/hello?wave&<name>")]
fn hello(name: Option<String>) -> String {
    name.map(|name|
        format!("HI, {}", name)
    ).unwrap_or_else(|| "hello".into())
}


fn main() {
    rocket::ignite().mount("/", routes![handler, hello]).launch();
}