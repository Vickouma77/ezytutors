use actix_web::{HttpResponse, web};

use crate::iter4::{
    AppState, Course, EzytutorError, get_course_details_db, get_courses_for_tutor_db,
    post_new_course_db,
};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

pub async fn get_course_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, EzytutorError> {
    let tutor_id = params.into_inner();

    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzytutorError> {
    let path_params = params;
    let tutor_id: i32 = path_params.0;
    let course_id: i32 = path_params.1;

    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzytutorError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}
