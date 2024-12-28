use actix_web::{web, App, HttpServer};
// use dotenv::dotenv; // Uncomment if you want to load environment variables from a .env file
use std::env;
use crate::models::Collection1;
use crate::routes::config;  
mod errors; 

mod models;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables (uncomment if using dotenv)
    dotenv::dotenv().ok();
    
    // Retrieve MongoDB URI from environment variables
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    // Parse the MongoDB URI and connect to the MongoDB client
    let client_options = mongodb::options::ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();

    // Access the "db1" database and the "collection1" collection
    let db = client.database("db1");
    let collection = db.collection::<Collection1>("collection1");

    // Log a message indicating the server is running
    println!("Server is running on http://127.0.0.1:8080");

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))  // Pass the MongoDB collection to the route handlers
            .configure(config)  // Configure the routes for the app
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost:8080
    .run()
    .await
}
