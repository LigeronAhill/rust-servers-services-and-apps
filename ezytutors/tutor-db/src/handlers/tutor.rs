use actix_web::{web, HttpResponse};

use crate::{
    db_access::tutor::{
        delete_tutor_db, get_all_tutors_db, get_tutor_details_db, post_new_tutor_db,
        update_tutor_details_db,
    },
    errors::EzyTutorError,
    models::tutor::{NewTutor, UpdateTutor},
    state::AppState,
};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}
pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    get_tutor_details_db(&app_state.db, tutor_id.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}
pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor_db(&app_state.db, NewTutor::from(new_tutor))
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}
pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    update_tutor_details_db(
        &app_state.db,
        tutor_id.into_inner(),
        UpdateTutor::from(update_tutor),
    )
    .await
    .map(|tutor| HttpResponse::Ok().json(tutor))
}
pub async fn delete_tutor(
    app_state: web::Data<AppState>,
    tutor_id: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    delete_tutor_db(&app_state.db, tutor_id.into_inner())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}
