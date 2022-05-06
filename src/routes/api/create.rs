use std::sync::Arc;

use actix_web::{post, web, Responder, Result};
use sha1::{Digest, Sha1};
use tokio::fs::File;

use crate::{
    config,
    http::{UploadEntry, UploadState},
    util::{checker, random},
};

#[post("/api/c/{ext}")]
pub async fn create(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let ext = path.into_inner().0;
    checker::in_bounds(
        "Length of extension ",
        ext.len(),
        0,
        config::EXTENSION_MAX_LENGTH,
    )?;

    let map = &mut state.map.lock().await;

    let stream = random::random_b64(config::STREAM_ID_LENGTH);

    let file = File::create(state.tmp_dir.clone() + &stream)
        .await
        .map_err(|e| crate::error!(FS_CREATE, FILE, "Error while creating file: {}", e))?;

    let hasher = Sha1::new();

    let entry = UploadEntry { file, ext, hasher };

    println!("Created stream with ID: {}", &stream);

    map.insert(stream.clone(), entry);

    Ok(web::Json(json!({ "stream": stream })))
}
