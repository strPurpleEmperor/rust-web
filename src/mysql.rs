use sqlx::{MySql, MySqlPool, Pool};
use std::env;

pub async fn creat_conn() -> Pool<MySql> {
    let database_url = env::var("DATABASE_URL").expect("MY_APP_SECRET must be set");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");
    pool
}
