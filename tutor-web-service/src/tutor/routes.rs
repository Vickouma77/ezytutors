use actix_web::web;

use crate::tutor::{
    delete_course, delete_tutor, get_all_tutors, get_course_detail, get_course_for_tutor,
    get_tutor_details, health_check_handler, post_new_course, post_new_tutor,
    update_course_details, update_tutor_details,
};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
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

pub fn tutor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tutors")
            .route("/", web::post().to(post_new_tutor))
            .route("/", web::get().to(get_all_tutors))
            .route("/{tutor_id}", web::get().to(get_tutor_details))
            .route("/{tutor_id}", web::put().to(update_tutor_details))
            .route("/{tutor_id}", web::delete().to(delete_tutor)),
    );
}
