#![feature(proc_macro_hygiene, decl_macro)]

mod order;

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/test")]
fn test() -> &'static str{
    "Test"
}

#[get("/")]
fn index() -> Template{
    Template::render("index",())
}




fn main() {
    rocket::ignite()
        .mount("/",routes![test,index])
        .attach(Template::fairing())
        .launch();
}
