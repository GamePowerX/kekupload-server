use std::{sync::Arc, path::Path};

use actix_files::NamedFile;
use actix_web::{web, get, Result, Responder, http::header::{ContentDisposition, DispositionType, DispositionParam}};

use crate::{http::UploadState, util::checker, models::file};

#[get("/api/d/{id}/")]
pub async fn download(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let id = path.into_inner().0;

    let db_connection = &checker::get_con(&state.pool)?;

    if let Some(entry) = file::File::find(id, &db_connection).into_iter().next() {
        let filename = get_filename(entry.hash.clone(), entry.ext);

        let named_file = NamedFile::open(Path::new(state.upload_dir.as_str()).join(entry.hash))
            .map_err(|e| crate::error!(FS_OPEN, FILE, "Error while opening file: {}", e))?;

        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![
                DispositionParam::Filename(filename)
            ],
        };

        Ok(named_file.set_content_disposition(content_disposition))
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}

fn get_filename(hash: String, ext: String) -> String {
    if ext.eq("none") {
        return hash;
    } else {
        return hash + "." + ext.as_str();
    }
}