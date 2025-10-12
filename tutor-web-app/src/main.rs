use std::env;

use actix_files as fs;
use actix_web::{App, Error, HttpResponse, HttpServer, error, web};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    println!("Listening on: {}, open browser and visit have a try!", addr);
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/**"))
            .expect("Error initializing tera template");
        let tera_wrap = web::Data::new(tera);
        
        App::new()
            .app_data(tera_wrap.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(addr)?
    .run()
    .await
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl
        .render("page.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
