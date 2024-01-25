/*
* Created on Wed Jun 01 2022
*
* Copyright (c) 2024 GamePowerX
*/

use std::{io::Error, sync::Arc};

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sha1::Sha1;
use tokio::{fs::File, sync::Mutex};

use crate::{colors, database::PgPool, routes};

pub struct UploadState {
    pub map: Mutex<HashMap<String, UploadEntry>>,

    pub tmp_dir: String,
    pub upload_dir: String,
    pub web_dir: String,

    pub pool: PgPool,

    pub embed_description: String,
    pub embed_color: String,
    pub download_url: String,

    pub chunk_size: usize,
}

pub struct UploadEntry {
    pub file: File,
    pub ext: String,
    pub hasher: Sha1,
}

pub async fn main(state: Arc<UploadState>, address: String, port: u16) -> Result<(), Error> {
    println!(
        "{}START{} http on {}{}:{}",
        colors::LIGHT_BLUE,
        colors::RESET,
        colors::ORANGE,
        address,
        port
    );

    match HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // CORS
            .wrap(cors)
            // API
            .service(routes::api::create)
            .service(routes::api::upload)
            .service(routes::api::finish)
            .service(routes::api::download)
            .service(routes::api::download_chunk)
            .service(routes::api::length)
            .service(routes::api::remove)
            .service(routes::embed::embed)
            // WEB
            .service(routes::web::index)
            .service(routes::web::handle_all)
            .app_data(web::Data::new(state.clone()))
    })
    .bind((address, port))?
    .run()
    .await
    {
        Ok(v) => {
            println!("{}STOP{} http", colors::ORANGE, colors::RESET);
            Ok(v)
        }
        Err(error) => {
            println!(
                "{}ERROR{} while binding http: {}",
                colors::RED,
                colors::RESET,
                error
            );
            return Err(error);
        }
    }
}
