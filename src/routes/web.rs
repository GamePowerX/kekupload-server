use std::sync::Arc;

use actix_files::NamedFile;
use actix_web::{get, Responder, web, Result, error};

use crate::{http::UploadState};

#[get("/")]
pub async fn index(
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let path = format!("{}index.html", state.web_dir);
    Ok(NamedFile::open(path)?)
}

#[get("/{path:.*}")]
pub async fn handle_all(
    path: web::Path<(String,)>, 
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let path = format!("{}{}", state.web_dir, path.into_inner().0);
    println!("PATH: {}", path);
    NamedFile::open(path).map_err(|e| error::ErrorNotFound(format!("File not found {}", e)))
}