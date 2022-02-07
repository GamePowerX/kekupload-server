#[macro_use] extern crate rocket;

use std::io;
use std::collections::HashMap;
use std::sync::Mutex;

use dotenv::dotenv;
use std::env;

//use rocket::tokio::time::{sleep, Duration};
use rocket::config::LogLevel;
use rocket::State;
use rocket::data::{Data, ToByteUnit};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use sha1::{Sha1, Digest};

mod database;
mod random;

struct UploadState {
    map: Mutex<HashMap<String, UploadEntry>>
}

struct UploadEntry {
    file: File,
    ext: String,
    hasher: Sha1
}

#[get("/")]
fn index() -> &'static str {
    "KekUpload api made by KekOnTheWorld!"
}

#[post("/c/<ext>")]
async fn create(ext: String, state: &State<UploadState>) -> io::Result<String> {
    let id = random::random_b64(64);
    let file = File::create("tmp/".to_owned() + &id).await?;
    let hasher = Sha1::new();

    let entry = UploadEntry { file: file, ext, hasher };

    println!("Created stream with ID: {}", &id);

    state.map.lock().unwrap().insert(id.clone(), entry);
    
    return Ok(id);
}

#[post("/u/<id>/<hash>", data = "<data>")]
async fn upload(data: Data<'_>, id: String, hash: String, state: &State<UploadState>) -> io::Result<&'static str> {
    let bytes = data.open(512.kibibytes()).into_bytes().await?.into_inner();
    
    let map = &mut state.map.lock().unwrap();
    if let Some(entry) = map.get_mut(&id) {
        let file = &mut entry.file;

        println!("{}: {}", id, entry.ext);
        file.write(&bytes).await?;
    } else {
        println!("INVALID_ID");
    }
    return Ok("Lol");
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let base = env::var("base").unwrap_or("/".to_string());

    let port = env::var("port")
        .unwrap_or("8000".to_string())
        .parse()
        .unwrap_or(8000);

    database::establish_connection(env::var("dburl").expect("Missing dburl in .env"));

    let figment = rocket::Config::figment()
        .merge(("log_level", LogLevel::Critical))
        .merge(("port", port));

    println!("http://localhost:{}{}", port, base);

    rocket::custom(figment)
        .manage(UploadState { map: Mutex::new(HashMap::new()) })
        .mount(base, routes![index, create, upload])
}
