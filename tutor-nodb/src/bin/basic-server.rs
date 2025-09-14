use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;

fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// the handler constructs http response
async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Health check OK")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().configure(general_routes))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
