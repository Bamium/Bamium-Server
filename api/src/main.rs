mod api;
mod database;
mod models;

#[tokio::main]
async fn main() {
    // Initialize database
    let db = database::init_database().await;

    // Initialize webserver
    let server = api::init_webserver(db.clone());

    server.await;
}