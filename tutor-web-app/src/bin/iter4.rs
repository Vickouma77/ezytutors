use std::{env, str::from_utf8};

use actix_files as fs;
use actix_web::{App, Error, HttpResponse, HttpServer, error, web};
use awc::Client;
use serde::{Deserialize, Serialize};
use serde_json::{self, from_str};
use tera::Tera;

#[derive(Serialize, Debug, Deserialize)]
struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let client = Client::default();

    //Request builder
    let res = client
        .get("http:://localhost:3000/tutors/")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = from_utf8(&res.as_ref()).unwrap();
    let tutor_list: Vec<Tutor> = from_str(str_list).unwrap();

    let mut ctx = tera::Context::new();

    ctx.insert("tutors", &tutor_list);
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template Error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    println!("Listening on {}", addr);
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter3/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind(addr)?
    .run()
    .await
}
