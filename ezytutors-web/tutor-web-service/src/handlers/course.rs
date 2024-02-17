use crate::db_access::course::{
    delete_course_db, get_course_details_db, get_courses_for_tutor_db, post_new_course_db,
    update_course_details_db,
};
use crate::errors::EzyTutorError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|c| HttpResponse::Ok().json(c))
}
pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|c| HttpResponse::Ok().json(c))
}
pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}
pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}
pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use actix_web::http::StatusCode;
    use actix_web::{web, ResponseError};
    use sqlx::PgPool;
    use std::sync::Mutex;
    #[actix_web::test]
    async fn get_all_courses_success() {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool = PgPool::connect(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_course_detail_failure_test() {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = get_course_details(app_state, parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
    #[ignore]
    #[actix_web::test]
    async fn post_course_success() {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_course_msg = CreateCourse {
            tutor_id: 1,
            course_name: "Third course".into(),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("English".into()),
            course_structure: None,
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_web::test]
    async fn update_course_success() {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let update_course_msg = UpdateCourse {
            course_name: Some("Course name changed".into()),
            course_description: Some("This is yet another test course".into()),
            course_format: None,
            course_level: Some("Intermediate".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("German".into()),
            course_structure: None,
        };
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let update_param = web::Json(update_course_msg);
        let resp = update_course_details(app_state, update_param, parameters)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[ignore]
    #[actix_web::test]
    async fn delete_test_success() {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 5));
        let resp = delete_course(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_web::test]
    async fn delete_test_failure() {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = delete_course(app_state, parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
