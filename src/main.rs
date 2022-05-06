#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

extern crate dotenv;

use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use dotenv::dotenv;
use http::UploadState;
use tokio::sync::Mutex;
use std::env;

pub mod database;
pub mod schema;
pub mod routes;
pub mod util;
pub mod config;
pub mod models;
pub mod errors;
pub mod http;
pub mod colors;

fn clean_tmp<'a>(tmp: &'a str) {
    fs::remove_dir_all(tmp).expect("Failed to remove temp directory!");
    fs::create_dir(tmp).expect("Failed to create temp directory!");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let tmp_dir = env::var("tmp_dir")
        .unwrap_or("tmp/".to_owned());
        
    let upload_dir = env::var("upload_dir")
        .unwrap_or("upload/".to_owned());

    let web_dir = env::var("web_dir")
        .unwrap_or("web/".to_owned());

    let embed_description = env::var("embed_description")
        .unwrap_or("No description availlable".to_owned());

    let embed_color = env::var("embed_color")
        .unwrap_or("#ffffff".to_owned());

    let download_url = env::var("download_url")
        .unwrap_or("http://example.com/".to_owned());

    let port = env::var("port")
        .unwrap_or("6942".to_owned())
        .parse()
        .unwrap_or(6942);

    let address = env::var("address")
        .unwrap_or("0.0.0.0".to_owned());

    let chunk_size: usize = env::var("chunksize")
        .unwrap_or("2048".to_owned())
        .parse()
        .unwrap_or(2048) * 1024;

    // Clean temp directory
    clean_tmp(tmp_dir.as_str());

    let pool = database::establish_connection(env::var("DATABASE_URL").expect("Database url not set!"));

    let state = UploadState {
        map: Mutex::new(HashMap::new()),
        tmp_dir,
        upload_dir,
        web_dir,
        pool,
        embed_description,
        embed_color,
        download_url,
        chunk_size
    };

    let _result = http::main(Arc::new(state), address, port).await;
}