#[macro_use] extern crate rocket;

use std::io;

use std::collections::HashMap;

use dotenv::dotenv;
use std::env;
//use rocket::tokio::time::{sleep, Duration};
use rocket::config::LogLevel;
use rocket::State;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use md5::{Md5, Digest};

mod database;
mod random;

struct UploadState {
    map: HashMap<String, UploadEntry>
}

struct UploadEntry {
    file: File,
    ext: String,
    hasher: Md5
}

#[get("/")]
fn index() -> &'static str {
    "KekUpload api made by KekOnTheWorld!"
}

#[get("/c/<ext>")]
async fn create(ext: String, state: &State<UploadState>) -> io::Result<String> {
    let id = random::random_b64(64);
    let file = File::create("tmp/".to_owned() + &id).await?;
    let hasher = Md5::new();

    let entry = UploadEntry { file, ext, hasher };

    println!("Created stream with ID: {}", &id);

    state.map.insert(id.clone(), entry);
    
    return Ok(id);
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
        .manage(UploadState { map: HashMap::new() })
        .mount(base, routes![index, create])
}
