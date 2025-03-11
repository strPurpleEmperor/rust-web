use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::{json, Value};

#[post("/echo")]
pub async fn echo(req_body: web::Json<Value>) -> impl Responder {
    let received_data = req_body.as_object().cloned();
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
    HttpResponse::Ok().json(response)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn index() -> impl Responder {
    "Hello world!"
}
