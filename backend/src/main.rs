use actix_web::{get, middleware::{Compress, Logger}, web, App, HttpResponse, HttpServer, Responder};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

async fn index() -> impl Responder {
    let index = web::Path::from("index.html".to_string());
    serve_file(index).await
    // HttpResponse::Ok().content_type("text/html").body(include_str!("../dist/index.html"))
}

async fn serve_file(path: web::Path<String>) -> impl Responder {
    let file_path = path.into_inner();
    match Asset::get(&file_path) {
        Some(content) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&file_path)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(Cow::into_owned(content.data)),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = 9080;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .route("/", web::get().to(index))
            .service(hello)
            .route("/{filename:.*}", web::get().to(serve_file))
    })
    .bind((host, port))?
    .run()
    .await
}
