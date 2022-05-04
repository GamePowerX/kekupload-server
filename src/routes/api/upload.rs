use actix_web::{web, Responder, post, Result};
use sha1::{Digest, Sha1};
use futures::StreamExt;
use tokio::io::AsyncWriteExt;

use crate::http::UploadState;

#[post("/u/{stream}/{hash}")]
pub async fn upload(
    path: web::Path<(String, String)>,
    state: web::Data<UploadState>,
    mut payload: web::Payload,
) -> Result<impl Responder> {
    let (stream, hash) = path.into_inner();

    let map = &mut state.map.lock().await;

    if let Some(entry) = map.get_mut(&stream) {
        // Load body
        let mut body = web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            if (body.len() + chunk.len()) > state.chunk_size {
                return Err(crate::error!(OVERFLOW, CHUNK, "Chunk size exceeded").into());
            }
            body.extend_from_slice(&chunk);
        }

        let mut sh = Sha1::new();
        sh.update(&body);
        let chunk_hash = hex::encode(sh.finalize());

        if !chunk_hash.eq(&hash) {
            return Err(crate::error!(HASH_MATCH, HASH, "Hash doesn't match").into());
        }

        entry.hasher.update(&body);
        entry.file.write_all(&body).await.map_err(|e| crate::error!(FS_WRITE, FILE, "Error while writing file: {}", e))?;

        Ok(web::Json(json!({
            "success": true
        })))
    } else {
        Err(crate::error!(NOT_FOUND, STREAM, "Stream not found").into())
    }
}