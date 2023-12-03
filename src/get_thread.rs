use crate::connect_database;
use axum::Json;
use serde::Serialize;
use sqlx::{MySql, Pool};

#[derive(Serialize, sqlx::FromRow)]
pub struct Thread {
    pub thread_id: i32,
    pub title: String,
    pub subject: String,
    pub place: String,
    pub purpose: String,
    pub comment: String,
    pub url: String,
    pub user_id: i32,
    pub user_name: String,
    pub create_at: String,
}

async fn get_all_thread(db: Pool<MySql>) -> Vec<Thread> {
    //let db = connect_database::connect_db().await.ok().unwrap();

    sqlx::query_as::<_, Thread>("SELECT * FROM threads")
        .fetch_all(&db)
        .await
        .ok()
        .unwrap()
}

#[derive(Serialize)]
pub struct GetThreadsResponse {
    pub threads: Vec<Thread>,
}

pub async fn get_threads_handler() -> Json<GetThreadsResponse> {
    let db = connect_database::connect_db().await.unwrap();
    let threads = get_all_thread(db).await;
    Json(GetThreadsResponse { threads })
}
