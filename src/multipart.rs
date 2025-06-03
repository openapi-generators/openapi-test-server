use std::io::Read;

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text, json::Json};
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::log::debug;

use crate::PublicError;

#[allow(clippy::single_match_else, clippy::too_many_lines)]
pub(crate) async fn upload(
    MultipartForm(body): MultipartForm<Data>,
) -> Result<impl Responder, PublicError> {
    let files = body
        .files
        .into_iter()
        .map(|mut file| {
            let mut data = String::with_capacity(file.size);
            file.file.read_to_string(&mut data).map_err(|_err| {
                PublicError::invalid("file", "failed to read data as UTF-8 string")
            })?;
            Ok(File {
                data,
                name: file.file_name,
                content_type: file.content_type.map(|c| c.to_string()),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    debug!("files: {files:?}");

    Ok(HttpResponse::Ok().json(Response {
        a_string: body.a_string.into_inner(),
        description: body.description.map(Text::into_inner),
        times: body.times.into_iter().map(Text::into_inner).collect(),
        objects: body.objects.into_iter().map(Json::into_inner).collect(),
        files,
    }))
}

#[derive(Debug, MultipartForm)]
pub(crate) struct Data {
    a_string: Text<String>,
    description: Option<Text<String>>,
    times: Vec<Text<DateTime>>,
    objects: Vec<Json<AnObject>>,
    files: Vec<TempFile>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct DateTime(#[serde(with = "time::serde::rfc3339")] OffsetDateTime);

#[derive(Debug, Deserialize, Serialize)]
struct AnObject {
    an_int: u64,
    a_float: f64,
}

#[derive(Debug, Serialize)]
pub(crate) struct Response {
    a_string: String,
    description: Option<String>,
    times: Vec<DateTime>,
    objects: Vec<AnObject>,
    files: Vec<File>,
}

#[derive(Debug, Serialize)]
struct File {
    name: Option<String>,
    content_type: Option<String>,
    data: String,
}
