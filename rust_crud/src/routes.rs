use actix_web::{web, HttpResponse, Responder};
use crate::handlers::{insert_document, fetch_documents, fetch_user_by_id, update_document, delete_document}; // Import handler functions
use crate::models::Collection1;

pub fn config(cfg: &mut web::ServiceConfig) {
    // Define the routes and associate them with the appropriate handlers
    cfg.route("/create", web::post().to(create_document))  // Route for creating a new document
        .route("/read", web::get().to(read_documents))  // Route for fetching all documents
        .route("/read/{id}", web::get().to(fetch_user_by_id))  // Route for fetching a document by ID
        .route("/update/{id}", web::put().to(update_document_handler))  // Route for updating a document by ID
        .route("/delete/{id}", web::delete().to(delete_document_handler));  // Route for deleting a document by ID
}

// Handler to create a new document
async fn create_document(
    collection: web::Data<mongodb::Collection<Collection1>>,  // Collection passed from app data
    item: web::Json<Collection1>,  // JSON body input for the new document
) -> impl Responder {
    // Call the insert_document function to insert the document into the database
    match insert_document(&collection, item.name.clone(), item.age, item.game.clone()).await {
        Ok(_) => HttpResponse::Created().json(item.into_inner()),  // Return created document on success
        Err(_) => HttpResponse::InternalServerError().body("Failed to create document"),  // Return error on failure
    }
}

// Handler to fetch all documents
async fn read_documents(
    collection: web::Data<mongodb::Collection<Collection1>>,  // Collection passed from app data
) -> impl Responder {
    // Call the fetch_documents function to retrieve all documents from the database
    match fetch_documents(&collection).await {
        Ok(response) => response,  // Return the documents as a response
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch documents"),  // Return error on failure
    }
}

// Handler to update a document
async fn update_document_handler(
    collection: web::Data<mongodb::Collection<Collection1>>,  // Collection passed from app data
    id: web::Path<String>,  // Path parameter containing the document ID
    item: web::Json<Collection1>,  // JSON body with updated fields for the document
) -> impl Responder {
    // Call the update_document function to update the document in the database
    match update_document(
        &collection,
        id.into_inner(),
        Some(item.name.clone()),  // Pass the updated name if provided
        Some(item.age),  // Pass the updated age if provided
    )
    .await
    {
        Ok(response) => response,  // Return the response from the update
        Err(_) => HttpResponse::InternalServerError().body("Failed to update document"),  // Return error on failure
    }
}

// Handler for Delete
async fn delete_document_handler(
    collection: web::Data<mongodb::Collection<Collection1>>,  // Collection passed from app data
    id: web::Path<String>,  // Path parameter containing the document ID
) -> impl Responder {
    // Call the delete_document function to delete the document from the database
    match delete_document(&collection, id.into_inner()).await {
        Ok(response) => response,  // Return the response from the delete operation
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete document"),  // Return error on failure
    }
}
