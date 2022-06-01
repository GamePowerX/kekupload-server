use std::{path::Path, sync::Arc, fs::File};

use actix_files::NamedFile;
use actix_web::{
    get,
    http::{header::{ContentDisposition, DispositionParam, DispositionType}, StatusCode},
    web, Responder, Result, HttpResponse
};

use crate::{
    http::UploadState,
    models::file,
    util::{
        checker::{self, map_qres},
        files,
        chunked
    },
};

#[get("/api/d/{id}")]
pub async fn download(
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
        let filename = files::get_filename(entry.hash.clone(), entry.ext);

        let named_file = NamedFile::open(Path::new(state.upload_dir.as_str()).join(entry.hash))
            .map_err(|e| crate::error!(FS_OPEN, FILE, "Error while opening file: {}", e))?;

        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(filename)],
        };

        Ok(named_file.set_content_disposition(content_disposition))
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}

#[get("/api/d/{id}/{offset}/{size}")]
pub async fn download_chunk(
    path: web::Path<(String, u64, u64,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let (id, offset, size) = path.into_inner();

    let db_connection = &checker::get_con(&state.pool)?;

    if let Some(entry) = map_qres(
        file::File::find(id, &db_connection),
        "Error while selecting files",
    )?
    .into_iter()
    .next()
    {
        let filename = files::get_filename(entry.hash.clone(), entry.ext);

        let file = File::open(Path::new(state.upload_dir.as_str()).join(entry.hash.clone()))
            .map_err(|e| crate::error!(FS_OPEN, FILE, "Error while opening file: {}", e))?;
        let file_size = file.metadata()
            .map_err(|e| crate::error!(FS_META, FILE, "Error while fetching metadata of file: {}", e))?.len();
        if file_size > offset + size {
            let content_disposition = ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(filename.clone())],
            };

            let mut response = HttpResponse::build(StatusCode::OK);

            response.insert_header(content_disposition);

            Ok(response.streaming(chunked::new_chunked_read(
                size,
                offset,
                file,
            )))
        } else {
            Err(crate::error!(RANGE, ID, "Download range is bigger than filesize").into())
        }
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}
