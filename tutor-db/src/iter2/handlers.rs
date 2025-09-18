use actix_web::{HttpResponse, web};

use crate::iter2::{AppState, Course};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    let reponse = format!(
        "Health check: {} visited {} times",
        health_check_response, visit_count
    );

    HttpResponse::Ok().json(&reponse)
}

pub async fn get_course_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<i32>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn get_course_detail(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn post_new_course(
    _new_course: web::Json<Course>,
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}