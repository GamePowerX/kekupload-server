/*
* Created on Wed Jun 01 2022
*
* Copyright (c) 2024 GamePowerX
*/

use actix_files::NamedFile;
use actix_web::{
    get,
    http::{
        header::{ContentDisposition, DispositionParam, DispositionType},
        StatusCode,
    },
    post, web, HttpResponse, Responder, Result,
};
use futures::StreamExt;
use sha1::{Digest, Sha1};
use std::{path::Path, sync::Arc};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{
    config,
    http::{UploadEntry, UploadState},
    models::file,
    util::{
        checker::{self, map_qres},
        chunked, files, random,
    },
};

// ANCHOR: create
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

// ANCHOR: upload
#[post("/api/u/{stream}")]
pub async fn upload(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
    mut payload: web::Payload,
) -> Result<impl Responder> {
    let (stream,) = path.into_inner();

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

        entry.hasher.update(&body);
        entry
            .file
            .write_all(&body)
            .await
            .map_err(|e| crate::error!(FS_WRITE, FILE, "Error while writing file: {}", e))?;

        Ok(web::Json(json!({
            "success": true
        })))
    } else {
        Err(crate::error!(NOT_FOUND, STREAM, "Stream not found").into())
    }
}

// ANCHOR: remove
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

// ANCHOR: finish
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
            fs::rename(file_path, state.upload_dir.clone() + &file_hash)
                .await
                .map_err(|e| crate::error!(FS_RENAME, FILE, "Error while renaming file: {}", e))?;

            let id = random::random_b64(config::FILE_ID_LENGTH);

            let db_connection = &mut checker::get_con(&state.pool).unwrap();

            let new_file = file::File {
                id: id.clone(),
                ext: entry.ext.clone(),
                hash: file_hash,
            };

            map_qres(new_file.create(db_connection), "Error while inserting file")?;

            Ok(web::Json(json!({ "id": id })))
        } else {
            Err(crate::error!(HASH_MATCH, HASH, "Hash doesn't match").into())
        }
    } else {
        Err(crate::error!(NOT_FOUND, STREAM, "Stream not found").into())
    }
}

// ANCHOR: download
#[get("/api/d/{id}")]
pub async fn download(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let id = path.into_inner().0;

    let db_connection = &mut checker::get_con(&state.pool)?;

    if let Some(entry) = map_qres(
        file::File::find(id, db_connection),
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

// ANCHOR: download_chunk
#[get("/api/d/{id}/{offset}/{size}")]
pub async fn download_chunk(
    path: web::Path<(String, u64, u64)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let (id, offset, size) = path.into_inner();

    let db_connection = &mut checker::get_con(&state.pool)?;

    if let Some(entry) = map_qres(
        file::File::find(id, db_connection),
        "Error while selecting files",
    )?
    .into_iter()
    .next()
    {
        let filename = files::get_filename(entry.hash.clone(), entry.ext);

        let file =
            std::fs::File::open(Path::new(state.upload_dir.as_str()).join(entry.hash.clone()))
                .map_err(|e| crate::error!(FS_OPEN, FILE, "Error while opening file: {}", e))?;
        let file_size = file
            .metadata()
            .map_err(|e| {
                crate::error!(
                    FS_META,
                    FILE,
                    "Error while fetching metadata of file: {}",
                    e
                )
            })?
            .len();
        if file_size > offset + size {
            let content_disposition = ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(filename.clone())],
            };

            let mut response = HttpResponse::build(StatusCode::OK);

            response.insert_header(content_disposition);

            Ok(response.streaming(chunked::new_chunked_read(size, offset, file)))
        } else {
            Err(crate::error!(RANGE, ID, "Download range is bigger than filesize").into())
        }
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}

// ANCHOR: length
#[get("/api/l/{id}")]
pub async fn length(
    path: web::Path<(String,)>,
    state: web::Data<Arc<UploadState>>,
) -> Result<impl Responder> {
    let id = path.into_inner().0;

    let db_connection = &mut checker::get_con(&state.pool)?;

    if let Some(entry) = map_qres(
        file::File::find(id, db_connection),
        "Error while selecting files",
    )?
    .into_iter()
    .next()
    {
        let file = File::open(Path::new(state.upload_dir.as_str()).join(entry.hash.clone()))
            .await
            .map_err(|e| crate::error!(FS_OPEN, FILE, "Error while opening file: {}", e))?;
        let file_size = file
            .metadata()
            .await
            .map_err(|e| {
                crate::error!(
                    FS_META,
                    FILE,
                    "Error while fetching metadata of file: {}",
                    e
                )
            })?
            .len();

        Ok(web::Json(json!({
            "size": file_size
        })))
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}
