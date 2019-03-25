#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

//use std::io;
//use std::os::unix::net::UnixStream;  // unix only >_<
use rocket::http::Status;
use rocket::response::{status, content, Stream};
use rocket_contrib::templates::Template;


#[post("/<id>")]
fn new(id: usize) -> status::Accepted<String> {
    println!("receive it {}", id);
    status::Accepted(Some(format!("id: '{}'", id)))
}

#[get("/json")]
fn json() -> content::Json<&'static str> {
    content::Json("{'hi': 'world'}")
}

#[get("/fail")]
fn custom_fail() -> Status {
    Status::NotAcceptable
}

//#[get("/stream")]
//fn stream() -> io::Result<Stream<UnixStream>> {
//    UnixStream::connect("/path/socket").map(|s| Stream::from(s))
//}
#[derive(Serialize, Deserialize)]
struct TemplateResponse {
    name: String,
    location: String,
}


#[derive(Serialize, Deserialize)]
struct ResponseNone {

}

#[get("/")]
fn index() -> Template {
    Template::render("index", ResponseNone{})
}


#[get("/tmp/<name>/<location>")]
fn tmp(name: String, location: String) -> Template {
    let context = TemplateResponse { name, location };
    Template::render("tem", &context)
}


fn main() {
    println!("Hello, world!");

    rocket::ignite().mount("/", routes![index, new, json, custom_fail, tmp])
        .attach(Template::fairing()).launch();
}