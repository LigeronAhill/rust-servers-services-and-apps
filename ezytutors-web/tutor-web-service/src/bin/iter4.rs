use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::io;
use std::sync::Mutex;
use tutor_web_service::{
    routes::{course_routes, general_routes},
    state::AppState,
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    let host_port = std::env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
