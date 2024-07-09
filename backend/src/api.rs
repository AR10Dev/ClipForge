use actix_web::get;

#[get("/hello")]
pub(crate) async fn hello() -> &'static str {
    "Hello, world!"
}
