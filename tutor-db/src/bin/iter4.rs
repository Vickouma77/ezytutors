use std::{env, io, sync::Mutex};

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::PgPool;
use tutor_db::iter4::{AppState, course_routes, general_routes};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let host_port = env::var("HOST_PORT").expect("HOST PORT is not set");

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    })
    .bind(host_port)?
    .run()
    .await
}
