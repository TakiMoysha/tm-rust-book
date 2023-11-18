use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(get_index)
            .route("/web:not_allowed", web::get().to(|| HttpResponse::MethodNotAllowed()))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
