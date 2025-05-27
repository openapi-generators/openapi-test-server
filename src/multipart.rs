use std::io::Read;

use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use actix_web::{HttpResponse, Responder};
use serde::Serialize;
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
        files,
    }))
}

#[derive(Debug, MultipartForm)]
pub(crate) struct Data {
    a_string: Text<String>,
    description: Option<Text<String>>,
    files: Vec<TempFile>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Response {
    a_string: String,
    description: Option<String>,
    files: Vec<File>,
}

#[derive(Debug, Serialize)]
struct File {
    name: Option<String>,
    content_type: Option<String>,
    data: String,
}
