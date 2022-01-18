#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]
#![allow(clippy::unused_async)]

use std::net::SocketAddr;

use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{http::StatusCode, Json, Router};
use serde::Serialize;

mod headers;
mod multipart;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/body/multipart", post(multipart::upload))
        .route("/parameters/header", post(headers::main))
        .route("/openapi.json", get(openapi));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn openapi() -> &'static str {
    include_str!("../openapi.json")
}

#[derive(Debug, Default, Serialize)]
struct PublicError {
    errors: Vec<String>,
    extra_parameters: Vec<String>,
    invalid_parameters: Vec<Problem>,
    missing_parameters: Vec<String>,
}

impl PublicError {
    pub(crate) fn missing(parameter: &str) -> Self {
        PublicError {
            missing_parameters: vec![String::from(parameter)],
            ..Self::default()
        }
    }

    pub(crate) fn invalid(parameter_name: &str, description: &str) -> Self {
        PublicError {
            invalid_parameters: vec![Problem::new(parameter_name, description)],
            ..Self::default()
        }
    }
}

#[derive(Debug, Serialize)]
struct Problem {
    parameter_name: String,
    description: String,
}

impl Problem {
    pub(crate) fn new(parameter_name: &str, description: &str) -> Self {
        Problem {
            parameter_name: String::from(parameter_name),
            description: String::from(description),
        }
    }
}

impl IntoResponse for PublicError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}
