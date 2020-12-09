#[macro_use] extern crate rocket;

mod paste_id;
#[cfg(test)] mod tests;

use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::fs;

use rocket::data::{Data, ToByteUnit};
use rocket::response::{content::Plain, Debug};
use rocket::tokio::fs::File;
use rocket::response::NamedFile;
// use rocket_contrib::serve::StaticFiles;


use crate::paste_id::PasteID;

const HOST: &str = "http://localhost:8000";
const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
async fn upload(paste: Data) -> Result<String, Debug<io::Error>> {
    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);

    paste.open(128.kibibytes()).stream_to_file(filename).await?;
    Ok(url)
}

#[get("/<id>",rank = 2)]
async fn retrieve(id: PasteID<'_>) -> Option<Plain<File>> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).await.map(Plain).ok()
}

#[get("/<file..>",rank = 3)]
async fn files(file: PathBuf) -> Result<(), Debug<io::Error>> {
    println!("The requested file path is {:?}", fs::canonicalize(&file));
    NamedFile::open(Path::new("./static/").join(file)).await?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, upload, retrieve,files])
    /*
    rocket::ignite()
    .mount("/", StaticFiles::from("/service/webserver/"))
    .launch() */
}
