use mongodb::Client;




pub async fn init_database() -> Client {
    // Initialize MongoDB
    Client::with_uri_str("mongodb://mongo:27017").await.unwrap()
}