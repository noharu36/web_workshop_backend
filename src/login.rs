use crate::connect_database;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::MySql;
use sqlx::Pool;

//Json変換
#[derive(Deserialize)]
pub struct ReceivedInfo {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Authentication {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub auth: bool,
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}


pub async fn check_user(n: String, p: String, db: Pool<MySql>) -> Option<User> {
    println!("{} {}", n, p);
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = ? AND password = ?")
        .bind(n)
        .bind(p)
        .fetch_one(&db)
        .await.ok()
}

pub async fn auth(Json(element): Json<ReceivedInfo>) -> Json<Authentication> {
    let db = connect_database::connect_db().await.unwrap();
    if let Some(usr) = check_user(element.name, element.password, db).await {
        println!("{}", usr.id);
        Json(Authentication {
            id: usr.id,
            name: usr.name,
            password: usr.password,
            auth: true,
        })
    } else {
        Json(Authentication {
            id: 0,
            name: String::from(""),
            password: String::from(""),
            auth: false,
        })
    }
}
