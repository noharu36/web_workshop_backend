use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use axum::Json;
use crate::connect_database;
use chrono::Local;

#[derive(Deserialize)]
pub struct ReceivedInfo {
    pub title: String,
    pub body: String,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Serialize)]
pub struct Thread {
    pub title: String,
    pub body: String,
    pub url: String,
    pub user_id: String,
    pub user_name: String,
    pub create_at: String
}

pub async fn create_thread(Json(element): Json<ReceivedInfo>) {
    let db = connect_database::connect_db().await;
    let coll = db.collection::<Thread>("threads");

    let mut url = String::from("thread");
    let cnt = (count_thread().await + 1).to_string();
    url += &cnt;

    let dt = Local::now();
    let time = dt.format("%Y-%m-%d %H:%M").to_string();

    let thread = Thread {
        title: element.title,
        body: element.body,
        url: url,
        user_id: element.user_id,
        user_name: element.user_name,
        create_at: time
    };

    coll.insert_one(thread, None).await.unwrap();
}

pub async fn count_thread() -> u64 {
    let db = connect_database::connect_db().await;
    let coll = db.collection::<Thread>("threads");

    coll.count_documents(None, None).await.ok().unwrap()
}