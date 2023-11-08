use mongodb::{Collection, Client, options::ClientOptions, bson::doc};
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use axum::Json;


//Json変換
#[derive(Serialize)]
pub struct Authentication {
    pub name: String,
    pub password: String,
    pub auth: bool
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

pub async fn connect_db() -> Collection::<User> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();


    client.database("test").collection::<User>("user")
}

pub async fn check_user(n: String, p: String) -> Option<User> {
    let coll = connect_db().await;
    let filter = doc!{"name": n, "password": p};
    let mut cursor = coll.find(filter, None).await.unwrap();

    while let Some(doc) = cursor.try_next().await.unwrap() {
        return Some(doc);
    }

    None

}

pub async fn auth(Json(element): Json<User>) -> Json<Authentication> {
    if let Some(usr) = check_user(element.name, element.password).await {
        Json(Authentication {
            name: usr.name,
            password: usr.password,
            auth: true })
    } else {
        Json(Authentication {
            name: String::from(""),
            password: String::from(""),
            auth: false
        })
    }
}