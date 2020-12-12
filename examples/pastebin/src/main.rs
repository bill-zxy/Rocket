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
use std::net::UdpSocket;

use rocket::Data;
use rocket::response::{content, Debug};
use rocket::response::content::Plain;
use rocket::response::NamedFile;
use paste_id::PasteID;
use rocket::config::{Config,Environment};

const HOST: &str = "http://localhost:8080";
const ID_LENGTH: usize = 3;

/*
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
*/
#[get("/<file..>")]

fn files(file: PathBuf) -> Option<NamedFile>  {
   NamedFile::open(Path::new("static/").join(file)).ok()
}


#[get("/")]
fn index() -> Option<NamedFile>  {
   NamedFile::open(Path::new("static/index.html")).ok()
}

 
fn rocket() -> rocket::Rocket {
    
    let ip = get_ipaddress().unwrap();
    let config = Config::build(Environment::Production)
    .address(ip)
    .port(8080) 
    .unwrap();
        
    rocket::custom(config).mount("/",routes![index, files])
}

fn main() {
    
    rocket().launch();
}



/// get the local ip address, return an `Option<String>`. when it fail, return `None`.
fn get_ipaddress() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}