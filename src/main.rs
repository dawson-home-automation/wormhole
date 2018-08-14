extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate json;

mod controllers;

use actix::System;
use actix_web::{middleware, server, App};
use controllers::*;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT").unwrap_or("8000".to_string());
    let sys = System::new("Wormhole");

    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/health", |r| r.f(health))
    }).bind(format!("0.0.0.0:{}", &port).as_str())
        .unwrap()
        .shutdown_timeout(1)
        .start();

    let _ = sys.run();
}
