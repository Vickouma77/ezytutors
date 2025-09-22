// Single Responsibility Principle
use sqlx::postgres::PgPool;
use crate::iter3::Course;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // SQL statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM
            ezy_course_c4 where tutor_id = $1",
            tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    // Extract rows
    course_row
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(
                course_row.posted_time.unwrap()
            )
            .into()),
        })
        .collect()
}