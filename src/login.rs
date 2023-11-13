use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use axum::Json;
use crate::connect_database;

//Json変換
#[derive(Deserialize)]
pub struct ReceivedInfo {
    pub name: String,
    pub password: String
}

#[derive(Serialize)]
pub struct Authentication {
    pub id: String,
    pub name: String,
    pub password: String,
    pub auth: bool
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,
}

pub async fn check_user(n: String, p: String) -> Option<User> {
    let db = connect_database::connect_db().await;
    let coll = db.collection::<User>("users");
    let filter = doc!{"name": n, "password": p};
    let mut cursor = coll.find(filter, None).await.unwrap();

    while let Some(doc) = cursor.try_next().await.unwrap() {
        return Some(doc);
    }

    None

}

pub async fn auth(Json(element): Json<ReceivedInfo>) -> Json<Authentication> {
    if let Some(usr) = check_user(element.name, element.password).await {
        Json(Authentication {
            id: usr.id.unwrap().to_string(),
            name: usr.name,
            password: usr.password,
            auth: true })
    } else {
        Json(Authentication {
            id: String::from(""),
            name: String::from(""),
            password: String::from(""),
            auth: false
        })
    }
}