mod api;
mod embed;
mod streaming;

use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use api::hello;
use embed::{dist, index};
use std::env;
use streaming::video_stream;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let host = "0.0.0.0";
    let port = 9080;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            // .wrap(Cors::default())
            .service(index)
            .service(hello)
            .service(video_stream)
            .service(dist)
    })
    .bind((host, port))?
    .run()
    .await
}
