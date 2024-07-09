use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[get("/{filename:.*}")]
pub(crate) async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}
