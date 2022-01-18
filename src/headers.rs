use axum::http::HeaderMap;
use axum::Json;
use serde::Serialize;

use crate::PublicError;

const STRING_HEADER_NAME: &str = "String-Header";
const INTEGER_HEADER_NAME: &str = "Integer-Header";
const NUMBER_HEADER_NAME: &str = "Number-Header";
const BOOLEAN_HEADER_NAME: &str = "Boolean-Header";

pub(crate) async fn main(mut headers: HeaderMap) -> Result<Json<Data>, PublicError> {
    let string = match headers.remove(STRING_HEADER_NAME) {
        Some(value) => value
            .to_str()
            .map_err(|_| PublicError::invalid(STRING_HEADER_NAME, "Not a valid string"))?
            .to_string(),
        None => return Err(PublicError::missing(STRING_HEADER_NAME)),
    };
    let integer = match headers.remove(INTEGER_HEADER_NAME) {
        Some(value) => {
            let str_value = value.to_str().map_err(|_| {
                PublicError::invalid(
                    INTEGER_HEADER_NAME,
                    "Header needs to be a string to parse a number from.",
                )
            })?;
            str_value
                .parse::<i32>()
                .map_err(|_| PublicError::invalid(INTEGER_HEADER_NAME, "Not a valid integer"))?
        }
        None => return Err(PublicError::missing(INTEGER_HEADER_NAME)),
    };
    let number = match headers.remove(NUMBER_HEADER_NAME) {
        Some(value) => {
            let str_value = value.to_str().map_err(|_| {
                PublicError::invalid(
                    NUMBER_HEADER_NAME,
                    "Header must be a string to parse a number from",
                )
            })?;
            str_value
                .parse::<f64>()
                .map_err(|_| PublicError::invalid(NUMBER_HEADER_NAME, "Not a valid number"))?
        }
        None => return Err(PublicError::missing(NUMBER_HEADER_NAME)),
    };
    let boolean = match headers.remove(BOOLEAN_HEADER_NAME) {
        Some(value) => {
            let str_value = value.to_str().map_err(|_| {
                PublicError::invalid(BOOLEAN_HEADER_NAME, "Header must be a string")
            })?;
            match str_value {
                "true" => true,
                "false" => false,
                _ => {
                    return Err(PublicError::invalid(
                        BOOLEAN_HEADER_NAME,
                        "Value must either be 'true' or 'false'",
                    ))
                }
            }
        }
        None => return Err(PublicError::missing(BOOLEAN_HEADER_NAME)),
    };

    Ok(Json(Data {
        string,
        integer,
        number,
        boolean,
    }))
}

#[derive(Debug, Serialize)]
pub(crate) struct Data {
    string: String,
    integer: i32,
    number: f64,
    boolean: bool,
}
