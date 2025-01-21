use actix_cors::Cors;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::{json, Value};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: web::Json<Value>) -> impl Responder {
    // 解析动态 JSON 数据
    let received_data = req_body.as_object().cloned();

    // 构建响应 JSON 数据
    let response = match received_data {
        Some(data) => json!({
            "status": "success",
            "received": data,
        }),
        None => json!({
            "status": "error",
            "message": "Invalid JSON format",
        }),
    };

    // 返回 JSON 响应
    HttpResponse::Ok().json(response)
}

async fn index() -> impl Responder {
    "Hello world!"
}
// 提供特定页面的服务
async fn serve_page(req: HttpRequest) -> impl Responder {
    let path = req.match_info().query("filename");
    let file_path = format!("./fd/app/dist/{}", path);

    // 尝试加载页面
    match std::fs::read_to_string(&file_path) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content),
        Err(_) => HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Page not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .service(web::scope("/api").route("/index.html", web::get().to(index)))
            .service(echo)
            // 提供 favicon.ico
            .route(
                "/favicon.ico",
                web::get().to(|| async { NamedFile::open("./fd/app/dist/favicon.ico") }),
            )
            // 静态资源服务 (例如 CSS 和 JS)
            .service(fs::Files::new("/static", "./fd/app/dist/static").show_files_listing())
            // 首页服务
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(std::fs::read_to_string("./fd/app/dist/index.html").unwrap())
                }),
            )
            // 其他页面服务
            .route("/{filename}", web::get().to(serve_page))
    })
    .bind(("0.0.0.0", 8050))?
    .run()
    .await
}
