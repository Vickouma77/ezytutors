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
    _params: web::Path<(i32, )>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not set");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState { 
            health_check_response: "".to_string(), 
            visit_count: Mutex::new(0), 
            db: pool
        });

        let tutor_id: web::Path<(i32, )> = web::Path::from((1, ));
        let res = get_course_for_tutor(app_state, tutor_id).await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not set");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState { 
            health_check_response: "".to_string(), 
            visit_count: Mutex::new(0), 
            db: pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let res = get_course_detail(app_state, params).await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not set");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState { 
            health_check_response: "".to_string(), 
            visit_count: Mutex::new(0), 
            db: pool,
        });

        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: NaiveDate::from_ymd_opt(2025, 9, 19),
        };

        let params: web::Json<Course> = web::Json(new_course_msg);
        let res = post_new_course(params, app_state).await;

        assert_eq!(res.status(), StatusCode::OK);
    }
}