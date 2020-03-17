#![feature(proc_macro_hygiene, decl_macro, try_trait)]



#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::io;
use rocket::response;
use rocket::Request;
use rocket::response::Responder;
use rocket::http::Status;


#[get("/")]
fn index() -> Template{
    Template::render("index",())
}

#[derive(Debug)]
enum FetchError{
    NotFound,
    InternalError(io::Error),
}
impl<'r> Responder<'r> for FetchError{
    fn respond_to(self, _:&Request)->response::Result<'r>{
        match self{
            Self::NotFound => Err(Status::NotFound),
            Self::InternalError(_) => Err(Status::InternalServerError),
        }
    }
}
impl From<io::Error> for FetchError{
    fn from(other:io::Error)->Self{
        match other.kind(){
            io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::InternalError(other),
        }
    }
}


#[get("/<file..>")]
fn client_code(file:PathBuf)->Result<NamedFile,FetchError>{
    Ok(dbg!(NamedFile::open(match file.to_str().ok_or(FetchError::NotFound)?{
        "client.js"=>PathBuf::from("../target/deploy/client.js"),
        "client.wasm"=>PathBuf::from("../target/deploy/client.wasm"),
        _ => return Err(FetchError::NotFound)
    }))?)
}

                    


fn main() {
    rocket::ignite()
        .mount("/",routes![
            index,
            client_code,
        ])
        .attach(Template::fairing())
        .launch();
}
