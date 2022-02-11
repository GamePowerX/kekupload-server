#[macro_use] extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;


use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Mutex};

use dotenv::dotenv;
use random::random_b64;
use rocket::response;
use std::env;

use rocket::config::LogLevel;
use rocket::State;
use rocket::http::Status;
use rocket::data::{Data, ToByteUnit, Limits, ByteUnit};
use rocket::response::status;
use rocket::fs::NamedFile;
use rocket::response::Responder;
use rocket::{Request, Response};

use sha1::{Sha1, Digest};

pub mod database;
pub mod random;
pub mod schema;

pub mod models;
use models::file;

struct UploadState {
    map: Mutex<HashMap<String, UploadEntry>>,
    tmp: String,
    upload: String,
    chunk_size: ByteUnit,
    datapool: database::PgPool,
}

struct UploadEntry {
    file: File,
    ext: String,
    hasher: Sha1
}


//----- START OF ROUTE CODE -----

#[get("/")]
fn index() -> &'static str {
    "KekUpload api made by KekOnTheWorld!"
}

#[post("/c/<ext>")]
fn create(ext: String, state: &State<UploadState>) -> String {
    let map = &mut state.map.lock().unwrap();

    let id = random::random_b64(64);
    let file = File::create(state.tmp.clone() + &id).unwrap();
    let hasher = Sha1::new();

    let entry = UploadEntry { file: file, ext, hasher };

    println!("Created stream with ID: {}", &id);

    map.insert(id.clone(), entry);
    
    return id;
}

#[post("/u/<id>/<hash>", data = "<data>")]
async fn upload(data: Data<'_>, id: String, hash: String, state: &State<UploadState>) -> io::Result<&'static str> {
    let bytes = data.open(state.chunk_size).into_bytes().await?.into_inner();

    let map = &mut state.map.lock().unwrap();
    if let Some(entry) = map.get_mut(&id) {
        let mut sh = Sha1::new();
        sh.update(&bytes);
        let chunk_hash = hex::encode(sh.finalize());

        if !chunk_hash.eq(&hash) {
            return Ok("INVALID_HASH");
        }

        let file = &mut entry.file;
        let hasher = &mut entry.hasher;
        
        file.write(&bytes).expect("File write error!");
        hasher.update(&bytes);
        return Ok("OK");
    } else {
        return Ok("INVALID_ID");
    }
}

#[post("/f/<id>/<hash>")]
async fn finish(id: String, hash: String, state: &State<UploadState>) -> status::Custom<String> {
    let map = &mut state.map.lock().unwrap();
    if let Some(entry) = map.get_mut(&id) {
        let file_hash = hex::encode(entry.hasher.clone().finalize());
        let file_path = state.tmp.clone() + &id;

        if file_hash.eq(&hash) {
            fs::rename(file_path, state.upload.clone() + &file_hash)
                .expect("File rename error!");

            let nid = random_b64(6);

            // TODO: postgre insert with all necesary data
            file::File {
                id: nid.clone(), 
                ext: entry.ext.clone(), 
                hash: file_hash
            }.create(
                &state.datapool
                .get()
                .expect("Error while connecting to database!")
            ).expect("Couldn't insert!");

            map.remove(&id);

            return status::Custom(Status::Ok, nid);
        } else {
            map.remove(&id);

            fs::remove_file(file_path)
                .expect("File remove error!");
            return status::Custom(Status::BadRequest, "INVALID_HASH".to_owned());
        }
    } else {
        return status::Custom(Status::BadRequest,"INVALID_ID".to_owned());
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Attachment<R>(pub Status, pub R, pub String);

// // Sets the status code of the response and then delegates the remainder of the
// impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for Attachment<R> {
//     fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
//         Response::build_from(self.1.respond_to(req)?)
//             .status(self.0)
//             .ok()
//     }
// }



#[get("/d/<id>")]
async fn download(id: String, state: &State<UploadState>) /*-> Attachment<String>*/ {
    // TODO: postgre select
    //NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/e/<id>")]
async fn embed(id: String, state: &State<UploadState>) -> io::Result<&'static str> {
    // TODO: postgre select
    Ok("NOT_IMPLEMENTED")
}

//----- END OF ROUTE CODE -----

fn clean_tmp(tmp: String) {
    let dir = tmp.as_str();
    fs::remove_dir_all(dir).expect("Failed to remove temp directory!");
    fs::create_dir(dir).expect("Failed to create temp directory!");
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let base = env::var("base").unwrap_or("/".to_owned());
    let tmp = env::var("tmp").unwrap_or("tmp/".to_owned());
    let upload = env::var("upload").unwrap_or("upload/".to_owned());

    let port = env::var("port")
        .unwrap_or("8000".to_owned())
        .parse()
        .unwrap_or(8000);

    let limits = Limits::default()
        .limit("bytes", 2.mebibytes());

    let datapool = database::establish_connection(env::var("DATABASE_URL").expect("Database url not set!"));

    clean_tmp(tmp.clone());

    database::establish_connection(env::var("DATABASE_URL").expect("Missing dburl in .env"));

    let figment = rocket::Config::figment()
        .merge(("log_level", LogLevel::Critical))
        .merge(("limits", limits))
        .merge(("port", port));

    println!("http://localhost:{}{}", port, base);

    rocket::custom(figment)
        .manage(UploadState { 
            map: Mutex::new(HashMap::new()),
            tmp: tmp,
            upload: upload,
            chunk_size: 2.mebibytes(),
            datapool: datapool
        })
        .mount(base, routes![
            index, 
            create, 
            upload,
            finish,
            embed,
            download
        ])
}
