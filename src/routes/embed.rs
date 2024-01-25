/*
* Created on Wed Jun 01 2022
*
* Copyright (c) 2024 GamePowerX
*/

use std::sync::Arc;

use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder, Result};

use crate::{
    http::UploadState,
    models::file,
    util::checker::{self, map_qres},
    util::files,
};

// Should change if you have different id lengths
#[get("/{id:[a-zA-Z0-9-_]{7}}")]
pub async fn embed(
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
        let id = entry.id;

        let filename = files::get_filename(entry.hash.clone(), entry.ext.clone());

        let download_url = state.download_url.clone() + &id;
        let description = state.embed_description.as_str();
        let color = state.embed_color.as_str();

        let image = if files::is_image(entry.ext) {
            format!(
                concat!(
                    "<meta property='og:image' content='{0}'>",
                    "<meta property='twitter:image' content='{0}'>"
                ),
                download_url
            )
        } else {
            "".to_owned()
        };

        Ok(HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(format!(
                concat!(
                    "<!DOCTYPE html>",
                    "<meta http-equiv=\"Refresh\" content=\"0; url='{0}'\" />",
                    "<meta charset='UTF-8'>",
                    "<meta property='og:type' content='website'>",
                    "<meta property='twitter:card' content='summary_large_image'>",
                    "<meta name='title' content='{1}'>",
                    "<meta property='og:title' content='{1}'>",
                    "<meta name='theme-color' content='{2}'>",
                    "<meta name='description' content='{3}'>",
                    "<meta property='og:description' content='{3}'>",
                    "<meta property='twitter:description' content='{3}'>",
                    "{4}"
                ),
                download_url, filename, color, description, image
            )))
    } else {
        Err(crate::error!(NOT_FOUND, ID, "File with id not found").into())
    }
}
