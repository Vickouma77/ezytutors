use actix_web::web;

use crate::iter2::{get_course_detail, get_course_for_tutor, health_check_handler};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{tutor_id}", web::get().to(get_course_for_tutor))
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail))
    );
}