use actix_files as fs;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tera::Tera;
use tutor_web_app::iter5::{AppState, app_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host_port = env::var("HOST_PORT").expect("HOSTPORT is not set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter5/**/*")).unwrap();
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::Data::new(tera))
            .app_data(shared_data.clone())
            .service(fs::Files::new("/static", "static/").show_files_listing())
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
