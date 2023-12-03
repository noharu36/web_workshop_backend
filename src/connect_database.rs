use dotenvy::dotenv;
use sqlx::{MySql, Pool, MySqlPool};
use std::{env, process};

pub async fn connect_db() -> Result<Pool<MySql>, sqlx::Error> {
    dotenv().ok();
    let database_url = match env::var("DATABASE_URL") {
        Ok(ok) => ok,
        Err(err) => {
            eprint!("Error: std::env said, {}\n", err);
            process::exit(1);
        }
    };


    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}
