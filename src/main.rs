use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
struct HelloInfo {
    from: String,
    to: String,
}

#[get("/name/{from}/{to}")]
async fn hello_name(info: web::Path<HelloInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from {} to {}", info.from, info.to))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(hello_name)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
