use std::{path::Path, sync::Arc, fs::File};

use actix_web::{
    get, web, Responder, Result
};

use crate::{
    http::UploadState,
    models::file,
    util::checker::{self, map_qres},
};

#[get("/api/l/{id}")]
pub async fn length(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let id = path.into_inner().0;

    let db_connection = &checker::get_con(&state.pool)?;

    if let Some(entry) = map_qres(
        file::File::find(id, &db_connection),
        "Error while selecting files",
    )?
    .into_iter()
    .next()
    {
        let file = File::open(Path::new(state.upload_dir.as_str()).join(entry.hash.clone()))
            .map_err(|e| crate::error!(FS_OPEN, FILE, "Error while opening file: {}", e))?;
        let file_size = file.metadata()
            .map_err(|e| crate::error!(FS_META, FILE, "Error while fetching metadata of file: {}", e))?.len();
        
        Ok(file_size.to_string())
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}
