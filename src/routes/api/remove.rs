use std::sync::Arc;

use actix_web::{post, web, Responder, Result};
use tokio::fs;

use crate::http::UploadState;

#[post("/api/r/{stream}")]
pub async fn remove(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let stream = path.into_inner().0;

    let map = &mut state.map.lock().await;

    match map.remove(&stream) {
        Some(_) => {
            let file_path = state.tmp_dir.clone() + &stream;
            fs::remove_file(file_path)
                .await
                .map_err(|e| crate::error!(FS_REMOVE, FILE, "Error while removing file: {}", e))?;

            Ok(web::Json(json!({
                "success": true
            })))
        }
        None => Err(crate::error!(NOT_FOUND, STREAM, "Stream not found").into()),
    }
}
