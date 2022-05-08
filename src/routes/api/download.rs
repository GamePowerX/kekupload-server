use std::{path::Path, sync::Arc};

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web, Responder, Result,
};

use crate::{
    http::UploadState,
    models::file,
    util::{
        checker::{self, map_qres},
        files,
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
