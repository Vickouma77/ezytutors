use actix_web::{Error, HttpResponse, web};
use serde_json::json;

use crate::iter6::{AppState, EzyTutorError, NewCourse, NewCourseResponse, UpdateCourse};

pub async fn show_new_course_form(
    tmpl: web::Data<tera::Tera>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let tutor_id = path.into_inner();
    let mut ctx = tera::Context::new();
    ctx.insert("tutor_id", &tutor_id);
    ctx.insert("error", "");
    ctx.insert("current_course_name", "");
    ctx.insert("current_course_description", "");
    ctx.insert("current_course_format", "");
    ctx.insert("current_course_duration", "");
    ctx.insert("current_course_structure", "");
    ctx.insert("current_course_price", "");
    ctx.insert("current_course_language", "");
    ctx.insert("current_course_level", "");

    let s = tmpl
        .render("new-course.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_insert_course(
    tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    path: web::Path<i32>,
    params: web::Json<NewCourse>,
) -> Result<HttpResponse, Error> {
    let tutor_id = path.into_inner();
    let new_course = json!({
        "tutor_id": tutor_id,
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_structure": &params.course_structure,
        "course_duration": &params.course_duration,
        "course_price": &params.course_price,
        "course_language": &params.course_language,
        "course_level": &params.course_level
    });

    let awc_client = awc::Client::default();
    let mut response = awc_client
        .post("http://localhost:3000/courses/")
        .send_json(&new_course)
        .await
        .unwrap();

    if response.status().is_success() {
        let res = response.body().await?;
        let course_response: NewCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;

        let mut ctx = tera::Context::new();
        ctx.insert("tutor_id", &tutor_id);
        ctx.insert("title", "Course Successfully Added!");
        ctx.insert(
            "message",
            "Your course has been created and is now available.",
        );
        ctx.insert("course_id", &course_response.course_id);
        ctx.insert("course_name", &course_response.course_name);
        ctx.insert("course_description", &course_response.course_description);
        ctx.insert("course_format", &course_response.course_format);
        ctx.insert("course_duration", &course_response.course_duration);
        ctx.insert("course_structure", &course_response.course_structure);
        ctx.insert("course_price", &course_response.course_price);
        ctx.insert("course_language", &course_response.course_language);
        ctx.insert("course_level", &course_response.course_level);
        ctx.insert("posted_time", &course_response.posted_time);

        let s = tmpl
            .render("course-success.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;

        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("tutor_id", &tutor_id);
        ctx.insert("error", "Failed to create course. Please try again.");
        ctx.insert("current_course_name", &params.course_name);
        ctx.insert("current_course_description", &params.course_description);
        ctx.insert("current_course_format", &params.course_format);
        ctx.insert("current_course_duration", &params.course_duration);
        ctx.insert(
            "current_course_structure",
            &params.course_structure.as_ref().unwrap_or(&"".to_string()),
        );
        ctx.insert(
            "current_course_price",
            &params.course_price.unwrap_or(0).to_string(),
        );
        ctx.insert(
            "current_course_language",
            &params.course_language.as_ref().unwrap_or(&"".to_string()),
        );
        ctx.insert(
            "current_course_level",
            &params.course_level.as_ref().unwrap_or(&"".to_string()),
        );

        let s = tmpl
            .render("new-course.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;

        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}

pub async fn handle_update_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    params: web::Json<UpdateCourse>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Got update request"))
}

pub async fn handle_delete_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Got delete request"))
}
