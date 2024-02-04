use std::{env, io};
use chrono::NaiveDateTime;
use sqlx::{PgPool, query};

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let course_rows = query!(
        r#"select course_id, tutor_id, course_name, posted_time from ezy_course_c4 where course_id = $1"#,
        1
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();
    let mut courses_list = Vec::new();
    for course_row in course_rows {
        courses_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: course_row.posted_time
        });
    }
    println!("Courses\n{courses_list:#?}");
    Ok(())
}