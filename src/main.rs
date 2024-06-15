use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
    message: String,
}

async fn index() -> impl Responder {
    web::Json(MyResponse {
        message: "Hello world!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
