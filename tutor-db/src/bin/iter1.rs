use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::PgPool;
use std::{env, io::Result};

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> Result<()>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database Url is not set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    
    let course_id: i32 = env::var("COURSE_ID")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1); // fallback to 1 if not set

    let course_rows = sqlx::query!(
        r#"
            select course_id, tutor_id, course_name, posted_time 
            from ezy_course_c4 
            where course_id = $1
        "#,
        course_id
    )
    .fetch_all(&db_pool)
    .await
    .expect("Failed to fetch courses from the database");

    let mut courses_list = vec![];

    for course_row in course_rows {
        courses_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: course_row.posted_time,
        })
    }
    println!("Courses = {:?}", courses_list);
    Ok(())
}