extern crate actix_web;

mod controllers;

use controllers::*;
use std::env;
use actix_web::{server, App};


fn main() {
    let port = env::var("PORT").unwrap_or("8000".to_string());

    server::new(|| {
        App::new().resource("/health", |r| r.f(health))
    })
    .bind(format!("0.0.0.0:{}", &port).as_str())
    .expect(format!("Can not bind to port {}", port).as_str())
    .run();
}
