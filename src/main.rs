mod heic;
mod mysql;
mod pages;
mod tools;
mod users;

use crate::mysql::creat_conn;
use crate::pages::serve_page;
use crate::tools::{echo, index};
use crate::users::get_users;
use actix_cors::Cors;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{web, App, HttpResponse, HttpServer};
use crate::heic::convert_heic;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = creat_conn().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .service(
                web::scope("/api")
                    .route("/index.html", web::get().to(index))
                    .service(get_users)
                    .service(convert_heic),
            )
            .service(echo)
            .route(
                "/favicon.ico",
                web::get().to(|| async { NamedFile::open("./fd/app/dist/favicon.ico") }),
            )
            .service(fs::Files::new("/static", "./fd/app/dist/static").show_files_listing())
            .route(
                "/",
                web::get().to(|| async {
                    match std::fs::read_to_string("./fd/app/dist/index.html") {
                        Ok(content) => HttpResponse::Ok()
                            .content_type("text/html; charset=utf-8")
                            .body(content),
                        Err(_) => HttpResponse::InternalServerError()
                            .content_type("text/plain")
                            .body("Failed to load index page"),
                    }
                }),
            )
            .route("/{filename}", web::get().to(serve_page))
    })
    .bind(("0.0.0.0", 8050))?
    .run()
    .await
}
