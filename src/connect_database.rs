use mongodb::{Client, options::ClientOptions, Database};

pub async fn connect_db() -> Database {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();


    client.database("sns_db")
}