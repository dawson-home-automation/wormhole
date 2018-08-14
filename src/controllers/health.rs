use actix_web::{HttpRequest, Responder};

pub fn health(_: &HttpRequest) -> impl Responder {
    "{\"status\": \"ok\"}"
}