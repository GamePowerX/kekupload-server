/*
* Created on Wed Jun 01 2022
*
* Copyright (c) 2024 GamePowerX
*/

use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::{error, get, web, Responder, Result};

use crate::http::UploadState;

#[get("/")]
pub async fn index(state: web::Data<Arc<UploadState>>) -> Result<impl Responder> {
    let path = format!("{}index.html", state.web_dir);
    Ok(NamedFile::open(path)?)
}

#[get("/{path:.*}")]
pub async fn handle_all(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let path = format!("{}{}", state.web_dir, path.into_inner().0);
    match NamedFile::open(path) {
        Ok(file) => Ok(file),
        Err(_) => NamedFile::open(format!("{}404.html", state.web_dir)).map_err(|e| {
            error::ErrorNotFound(format!("File not found and 404.html doesn't exist: {}", e))
        }),
    }
}
