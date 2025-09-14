use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use tutor_nodb::state::AppState;
use tutor_nodb::routes::*;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good, you've already asked me".to_string(),
        visit_count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}