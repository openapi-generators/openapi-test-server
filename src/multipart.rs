use axum::extract::multipart::MultipartError;
use axum::extract::Multipart;
use axum::Json;
use serde::Serialize;

use crate::{Problem, PublicError};

impl From<MultipartError> for PublicError {
    fn from(e: MultipartError) -> Self {
        PublicError {
            errors: vec![format!("Invalid multipart request: {}", e)],
            ..Self::default()
        }
    }
}

#[allow(clippy::single_match_else, clippy::too_many_lines)]
pub(crate) async fn upload(mut multipart: Multipart) -> Result<Json<File>, PublicError> {
    let mut err: Option<PublicError> = None;
    let mut a_string = None;
    let mut description = None;
    let mut file_content_type = None;
    let mut file_name = None;
    let mut file_data = None;

    while let Some(field) = multipart.next_field().await? {
        let field_name = match field.name() {
            Some(name) => name,
            None => {
                err.get_or_insert_with(Default::default)
                    .extra_parameters
                    .push(String::from("unnamed parameter"));
                continue;
            }
        };

        match field_name {
            "a_string" => {
                if let Ok(value) = field.text().await {
                    a_string = Some(value);
                } else {
                    err.get_or_insert_with(Default::default)
                        .invalid_parameters
                        .push(Problem {
                            parameter_name: String::from("a_string"),
                            description: String::from("must be a string"),
                        });
                }
            }
            "description" => {
                if let Ok(value) = field.text().await {
                    description = Some(value);
                } else {
                    err.get_or_insert_with(Default::default)
                        .invalid_parameters
                        .push(Problem {
                            parameter_name: String::from("description"),
                            description: String::from("must be a string"),
                        });
                }
            }
            "file" => {
                if let Some(value) = field.file_name() {
                    file_name = Some(value.to_string());
                } else {
                    err.get_or_insert_with(Default::default)
                        .invalid_parameters
                        .push(Problem {
                            parameter_name: String::from("file"),
                            description: String::from("must have a file name"),
                        });
                }

                if let Some(value) = field.content_type() {
                    file_content_type = Some(value.to_string());
                } else {
                    err.get_or_insert_with(Default::default)
                        .invalid_parameters
                        .push(Problem {
                            parameter_name: String::from("file"),
                            description: String::from("must have a content type"),
                        });
                }

                if let Ok(value) = field.bytes().await {
                    file_data = Some(value);
                } else {
                    err.get_or_insert_with(Default::default)
                        .invalid_parameters
                        .push(Problem {
                            parameter_name: String::from("file"),
                            description: String::from("must have data"),
                        });
                }
            }
            field_name => {
                err.get_or_insert_with(Default::default)
                    .extra_parameters
                    .push(String::from(field_name));
            }
        }
    }

    let a_string = match a_string {
        Some(name) => name,
        None => {
            let mut err = err.unwrap_or_default();
            err.missing_parameters.push(String::from("a_string"));
            return Err(err);
        }
    };

    let (file_content_type, file_name, file_data) = match (file_content_type, file_name, file_data)
    {
        (Some(file_content_type), Some(file_name), Some(file_data)) => {
            (file_content_type, file_name, file_data.to_vec())
        }
        _ => {
            let mut err = err.unwrap_or_default();
            err.missing_parameters.push(String::from("file"));
            return Err(err);
        }
    };

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(Json(File {
            a_string,
            description,
            file_content_type,
            file_name,
            file_data: String::from_utf8_lossy(&file_data).to_string(),
        }))
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct File {
    a_string: String,
    description: Option<String>,
    file_content_type: String,
    file_name: String,
    file_data: String,
}
