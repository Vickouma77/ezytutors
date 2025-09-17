use crate::models::Course;

use super::state::AppState;
use actix_web::{HttpResponse, web};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;
    let response = format!(
        "Health check: {} (visited {} times)",
        health_check_response, visit_count
    );

    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1).try_into().unwrap()),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added Course")
}

pub async fn get_course_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let tutor_id: i32 = params.into_inner();

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .iter()
        .filter(|course| course.tutor_id == tutor_id)
        .cloned()
        .collect::<Vec<Course>>();

    if !filtered_courses.is_empty() {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No course found for tutor".to_string())
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.tutor_id == tutor_id && x.course_id == Some(course_id))
        .ok_or("course not found");

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("course not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is to test the course".into(),
            course_id: None,
            posted_time: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let res = new_course(course, app_state).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let tutor_id: web::Path<i32> = web::Path::from(1);
        let res = get_course_for_tutor(app_state, tutor_id).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState { 
            health_check_response: "".to_string(), 
            visit_count: Mutex::new(0), 
            courses: Mutex::new(vec![]),
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let res = get_course_detail(app_state, params).await;

        assert_eq!(res.status(), StatusCode::OK)
    }
}
