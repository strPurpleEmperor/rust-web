use actix_web::{HttpRequest, HttpResponse, Responder};
use std::path::Path;

pub async fn serve_page(req: HttpRequest) -> impl Responder {
    let path = req.match_info().query("filename");
    let file_path = Path::new("./fd/app/dist").join(path);

    if !file_path.starts_with(Path::new("./fd/app/dist")) {
        return HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("Invalid path");
    }

    if let Ok(content) = std::fs::read_to_string(&file_path) {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content)
    } else {
        HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Page not found")
    }
}
