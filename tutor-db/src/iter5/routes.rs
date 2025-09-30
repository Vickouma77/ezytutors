use actix_web::web;

use crate::iter5::{
    delete_course, get_course_detail, get_course_for_tutor, health_check_handler, post_new_course,
    update_course_details,
};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/helath", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{tutor_id}", web::get().to(get_course_for_tutor))
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail))
            .route(
                "/{tutor_id}/{course_id}",
                web::put().to(update_course_details),
            )
            .route("/{tutor_id}/{course_id}", web::delete().to(delete_course)),
    );
}
