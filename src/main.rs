use std::fmt::{Display, Formatter};

use actix_web::{App, HttpResponse, HttpServer, ResponseError, web};
use serde::Serialize;

mod headers;
mod multipart;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            .route("/body/multipart", web::post().to(multipart::upload))
            .route("/parameters/header", web::post().to(headers::main))
            .route("/openapi.yaml", web::get().to(openapi))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

async fn openapi() -> &'static str {
    include_str!("../openapi.yaml")
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

impl Display for PublicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl ResponseError for PublicError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}
