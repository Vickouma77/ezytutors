use actix_web::{Error, HttpResponse};

pub async fn show_register_form() -> Result<HttpResponse, Error> {
    let msg = "Hello you are in the registration page";

    Ok(HttpResponse::Ok().content_type("text/html").body(msg))
}

pub async fn handle_register() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(""))
}
