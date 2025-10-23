use crate::iter6::{
    handle_delete_course, handle_insert_course, handle_register, handle_signin,
    handle_update_course, show_new_course_form, show_register_form, show_signin_form,
};
use actix_files as fs;
use actix_web::web;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(show_register_form)))
            .service(web::resource("/signinform").route(web::get().to(show_signin_form)))
            .service(web::resource("/signin").route(web::post().to(handle_signin)))
            .service(web::resource("/register").route(web::post().to(handle_register))),
    );
}

pub fn course_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .service(
                web::resource("/new/{tutor_id}")
                    .route(web::get().to(show_new_course_form))
                    .route(web::post().to(handle_insert_course)),
            )
            .service(
                web::resource("/{tutor_id}/{course_id}").route(web::put().to(handle_update_course)),
            )
            .service(
                web::resource("/delete/{tutor_id}/{course_id}")
                    .route(web::delete().to(handle_delete_course)),
            ),
    );
}
