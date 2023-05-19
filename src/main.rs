use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mysql::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(oi)
            .route("/hey", web::get().to(hey))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/product")]
async fn oi() -> impl Responder {
    HttpResponse::Ok().body("oi.")
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
