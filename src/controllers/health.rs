use actix_web::{HttpRequest, HttpResponse, Responder};

#[derive(Serialize)]
struct Response {
    status: String,
}

pub fn health(_: &HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
    })
}
