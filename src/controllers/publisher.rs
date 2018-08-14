use actix_web::{HttpResponse, Responder, Json};

#[derive(Debug, Deserialize)]
pub struct Request {
    topic: String,
    payload: String
}

pub fn publish(payload: Json<Request>) -> impl Responder {
    println!("{:?}", payload);

    HttpResponse::Ok()
}
