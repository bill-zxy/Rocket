#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod paste_id;
#[cfg(test)] mod tests;

use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;

use rocket::data::Data;
use rocket::response::{content::Plain, Debug};
// use rocket::tokio::fs::File;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket::config::{Config,Environment};
use std::fs::File;0.3:8080";
const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
async fn upload(paste: Data) -> Result<String, Debug<io::Error>> {
    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}", host = HOST, id = id);

    paste.open(128.kibibytes()).stream_to_file(filename).await?;
    Ok(url)
}

#[get("/<id>",rank = 2)]
async fn retrieve(id: PasteID<'_>) -> Option<Plain<File>> {
    let filename = format!("upload/{id}", id = id);
    NamedFile::open(&filename).await.map(Plain).ok()
}

#[get("/<file..>", rank =3)]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()));
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join("index.html");
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()));
}


fn main() {
    
    let config = Config::build(Environment::Production)
     .address("172.17.0.3")
     .port(8080)
     .finalize();

    rocket::ignite().mount("/static", routes![index, upload, retrieve,files]).launch();
}
