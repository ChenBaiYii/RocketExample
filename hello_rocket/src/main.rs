#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod video_scope {
    #[get("/video")]
    pub fn index() -> &'static str {
        "<html><h1>testing</h1></html>"
    }
}

mod music_scope {
    #[get("/music")]
    pub fn music() -> &'static str {
        "<html><h1>testing music</h1></html>"
    }
}

mod show_article {
	#[get("/article")]
	
}


fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", routes![video_scope::index, music_scope::music]).launch();
}