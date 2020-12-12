#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
extern crate rand;

mod paste_id;
#[cfg(test)] mod tests;

use std::io;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;

use rocket::Data;
use rocket::response::{content, Debug};
use rocket::response::content::Plain;
use rocket::response::NamedFile;
use paste_id::PasteID;
use rocket::config::{Config,Environment};


const HOST: &str = "http://localhost:8080";
const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, Debug<io::Error>> {
    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>",rank = 2)]
fn retrieve(id: PasteID<'_>) -> Option<content::Plain<File>> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).map(|f|Plain(f)).ok()
}

#[get("/<file..>", rank =3)]

fn files(file: PathBuf) -> Option<NamedFile>  {
   NamedFile::open(Path::new("/service/webserver/static/").join(file)).ok()
}


#[get("/")]
fn index() -> Option<NamedFile>  {
   NamedFile::open(Path::new("/service/webserver/static/index.html")).ok()
}

 
fn rocket() -> rocket::Rocket {
    
    let config = Config::build(Environment::Production)
     .unwrap();
        
    rocket::custom(config).mount("/",routes![index, upload, retrieve,files])
}

fn main() {
    
    rocket().launch();
}