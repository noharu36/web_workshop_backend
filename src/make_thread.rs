use crate::connect_database;
use axum::Json;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ReceivedInfo {
    pub title: String,
    pub subject: String,
    pub place: String,
    pub purpose: String,
    pub comment: String,
    pub user_id: i32,
    pub user_name: String,
}

#[derive(Serialize)]
pub struct Thread {
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

pub async fn create_thread(Json(element): Json<ReceivedInfo>) {
    let db = connect_database::connect_db().await.unwrap();
    let mut url = String::from("thread");
    let cnt = (count_thread().await + 1).to_string();
    url += &cnt;

    let dt = Local::now();
    let time = dt.format("%Y-%m-%d %H:%M").to_string();

    sqlx::query!(
        r#"
            INSERT INTO threads(title, subject, place, purpose, comment, url, user_id, user_name, create_at)
            VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        element.title,
        element.subject,
        element.place,
        element.purpose,
        element.comment,
        url,
        element.user_id,
        element.user_name,
        time
    ).execute(&db).await.ok();
}

pub async fn count_thread() -> i64 {
    let db = connect_database::connect_db().await.ok().unwrap();
    sqlx::query!("SELECT COUNT(*) as count FROM threads")
        .fetch_one(&db)
        .await
        .unwrap()
        .count
}

//curlのコマンド curl localhost:8080/create -XPOST -H 'Content-Type: application/json' -d '{"title": "test-thread", "subject": "数学", "place": "UBIC", "purpose": "期末対策", "comment": "てすと", "user_id": 1, "user_name": "haru"}'
