use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;

    HttpServer::new(|| App::new().service(index).route("/hey", web::get().to(hey)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
