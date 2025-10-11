mod tutor;

use std::{env, io, sync::Mutex};

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use sqlx::PgPool;
use tutor::{AppState, course_routes, general_routes, tutor_routes};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let host_port = env::var("HOST_PORT").expect("HOST PORT is not set");
    let bind_address = format!("127.0.0.1:{}", host_port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
