use super::app_error::AppError;
use super::models::Course;
use sqlx::postgres::PgPool;

pub async
fn new_course_db(pool: &PgPool, new_course: Course, ) -> Result<Course, AppError> {
    let row = sqlx::query_as!(
        Course,
        r#"
            INSERT INTO course (
                tutor_id, course_name, course_description, course_duration, course_level,
                course_format, course_language, course_structure, course_price
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                tutor_id, course_id, course_name, course_description, course_duration,
                course_level, course_format, course_language, course_structure, course_price,
                posted_time
        "#,
        new_course.tutor_id,
        new_course.course_name,
        new_course.course_description,
        new_course.course_duration,
        new_course.course_level,
        new_course.course_format,
        new_course.course_language,
        new_course.course_structure,
        new_course.course_price
    )
    .fetch_one(pool)
    .await?;

    // 결과를 꺼낸다
    Ok( Course {
        course_id:          row.course_id,
        tutor_id:           row.tutor_id,
        course_name:        row.course_name.clone(),
        posted_time:        Some(row.posted_time.unwrap()),
        course_description: row.course_description,
        course_duration:    row.course_duration,
        course_level:       row.course_level,
        course_format:      row.course_format,
        course_language:    row.course_language,
        course_structure:   row.course_structure,
        course_price:       row.course_price,
    })
}


pub async
fn get_courses_db(pool: &PgPool, tutor_id: i32) ->  Result<Vec<Course>, AppError> {
    // SQL 구문을 준비한다
    let records: Vec<Course> = sqlx::query_as!(
        Course,
        r#"
            SELECT *
            FROM course
            WHERE tutor_id = $1
            ORDER BY course_id DESC
        "#,
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}


pub async
fn get_course_detail_db(pool: &PgPool, tutor_id: i32, course_id: i32) ->  Result<Course, AppError> {
    // SQL 구문을 준비한다
    let rows = sqlx::query!(
        r#"
            SELECT
                tutor_id, course_id, course_name, posted_time, course_description,
                course_duration, course_level, course_format, course_language, course_structure,
                course_price
            FROM
                course
            where
                tutor_id = $1 and course_id = $2
        "#,
        tutor_id,
        course_id,
    )
    .fetch_one(pool)
    .await;

    match rows {
        Ok(row) => {
            Ok(Course {
                course_id:          row.course_id,
                tutor_id:           row.tutor_id,
                course_name:        row.course_name,
                posted_time:        Some(chrono::NaiveDateTime::from(row.posted_time.unwrap())),
                course_description: row.course_description,
                course_duration:    row.course_duration,
                course_level:       row.course_level,
                course_format:      row.course_format,
                course_language:    row.course_language,
                course_structure:   row.course_structure,
                course_price:       row.course_price,
            })
        }
        Err(_) => {
            Err(AppError::NotFound("Course id not found".into()))
        }
    }
}

// 강의를 삭제한다
pub async
fn delete_course_db(pool: &PgPool, tutor_id: i32, course_id: i32, ) -> Result<String, AppError> {
    // SQL 구문을 준비한다
    let rows_deleted = sqlx::query!(
        r#"
            DELETE
            FROM course
            where tutor_id = $1 and course_id = $2
        "#,
        tutor_id,
        course_id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:#?} record", rows_deleted))
}


