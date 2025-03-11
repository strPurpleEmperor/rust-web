use actix_web::{get, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, MySqlPool};
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
    user_id: i32,
    username: String,
    password: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    profile_image: Option<String>,
    bio: Option<String>,
    location: Option<String>,
    website: Option<String>,
    is_active: bool,
    is_admin: bool,
    last_login: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
#[get("/users")]
pub async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Database error: {}", e)
        })),
    }
}
