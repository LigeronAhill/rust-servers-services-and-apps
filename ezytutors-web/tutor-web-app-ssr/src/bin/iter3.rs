use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let tutors = vec![
        Tutor {
            name: "John Doe".to_string(),
        },
        Tutor {
            name: "Jane Doe".to_string(),
        },
        Tutor {
            name: "Joe Doe".to_string(),
        },
    ];
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);
    let s = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8000");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter3/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
