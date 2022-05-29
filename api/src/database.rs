use crate::models::Databases;


pub async fn init_database() -> Databases {
    // Initialize MongoDB
    let mongo = mongodb::Client::with_uri_str("mongodb://mongo:27017").await.unwrap();

    mongo.database("bamium").create_collection("users", None).await.ok();
    mongo.database("bamium").create_collection("channels", None).await.ok();
    mongo.database("bamium").create_collection("roles", None).await.ok();
    mongo.database("bamium").create_collection("messages", None).await.ok();

    // Initialize Redis
    let redis = redis::Client::open("redis://redis:6379").unwrap();

    Databases { mongo: mongo, redis: redis }
}