use std::sync::Arc;

use actix_web::{web, Responder, post, Result};
use sha1::Digest;
use tokio::fs;

use crate::{http::UploadState, util::{checker::{self, map_qres}, random}, config, models::file};


#[post("/api/f/{stream}/{hash}")]
pub async fn finish(
    path: web::Path<(String, String)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let (stream, hash) = path.into_inner();

    let map = &mut state.map.lock().await;

    if let Some(entry) = map.get_mut(&stream) {
        let file_hash = hex::encode(entry.hasher.clone().finalize());
        let file_path = state.tmp_dir.clone() + &stream;

        if file_hash.eq(&hash) {
            fs::rename(file_path, state.upload_dir.clone() + &file_hash).await.map_err(|e| crate::error!(FS_RENAME, FILE, "Error while renaming file: {}", e))?;

            let id = random::random_b64(config::FILE_ID_LENGTH);

            let db_connection = &checker::get_con(&state.pool)?;

            let new_file = file::File {
                id: id.clone(), 
                ext: entry.ext.clone(), 
                hash: file_hash
            };
            
            map_qres(new_file.create(db_connection), "Error while inserting file")?;

            Ok(web::Json(json!({
                "id": id
            })))
        } else {
            Err(crate::error!(HASH_MATCH, HASH, "Hash doesn't match").into())
        }
    } else {
        Err(crate::error!(NOT_FOUND, STREAM, "Stream not found").into())
    }
}