use actix_web::{web, HttpResponse, Responder};
use futures::stream::TryStreamExt; 
use mongodb::{bson::doc, Collection};
use crate::models::Collection1;
use mongodb::bson::oid::ObjectId;
use crate::errors::AppError; 

// Function to insert a new document into the collection
pub async fn insert_document(
    collection: &Collection<Collection1>,
    name: String,
    age: u32,
    game: String,
) -> Result<impl Responder, AppError> {
    // Validate input fields
    if name.trim().is_empty() {
        return Err(AppError::InvalidInput("Name cannot be empty".to_string())); // Return error if name is empty
    }

    if age == 0 {
        return Err(AppError::InvalidInput("Age cannot be zero".to_string())); // Return error if age is zero
    }

    let new_doc = Collection1 {
        name,
        age,
        game,
        // created_at: Some(mongodb::bson::DateTime::now()),
    };

    // Attempt to insert the document into the collection
    match collection.insert_one(new_doc.clone(), None).await {
        Ok(_) => {
            println!("Document inserted successfully!");
            Ok(HttpResponse::Created().json(new_doc)) // Return the created document as a response
        },
        Err(e) => {
            // Handle MongoDB error cases
            Err(AppError::DatabaseError(e.to_string())) // Map MongoDB error to AppError
        }
    }
}

// Function to fetch all documents from the collection
pub async fn fetch_documents(collection: &Collection<Collection1>) -> Result<HttpResponse, AppError> {
    // Fetch all documents using the MongoDB find query
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(err) => return Err(AppError::DatabaseError(err.to_string())), // Return DatabaseError if the query fails
    };

    let mut documents = Vec::new();

    // Iterate through the cursor to collect documents
    while let Some(result) = match cursor.try_next().await {
        Ok(doc) => doc,
        Err(err) => return Err(AppError::DatabaseError(err.to_string())), // Handle stream iteration failure
    } {
        documents.push(result);
    }

    // If no documents are found, return the DocumentNotFound error
    if documents.is_empty() {
        return Err(AppError::DocumentNotFound);
    }

    // Return the list of documents as JSON
    Ok(HttpResponse::Ok().json(documents))
}

// Function to fetch a user by their ID
pub async fn fetch_user_by_id(
    collection: web::Data<Collection<Collection1>>,
    id: web::Path<String>,  // Path argument for user ID
) -> Result<impl Responder, AppError> {
    // Dereference the Path to get the inner String and parse ObjectId
    let obj_id = ObjectId::parse_str(&*id).map_err(|_| AppError::InvalidIdFormat(id.to_string()))?;

    let filter = doc! { "_id": obj_id };

    // Attempt to find the document in the collection
    match collection.find_one(filter, None).await {
        Ok(Some(doc)) => Ok(HttpResponse::Ok().json(doc)), // Return the document if found
        Ok(None) => Err(AppError::DocumentNotFound), // If no document is found, return DocumentNotFound error
        Err(err) => Err(AppError::DatabaseError(err.to_string())), // If a DB error occurs, return DatabaseError
    }
}

// Function to update a document in the collection
pub async fn update_document(
    collection: &Collection<Collection1>,
    id: String,
    updated_name: Option<String>,
    updated_age: Option<u32>,
) -> mongodb::error::Result<HttpResponse> {
    // Convert the string ID to ObjectId
    let object_id = match mongodb::bson::oid::ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid ID format")), // Return BadRequest if ID format is invalid
    };

    // Build the update document
    let mut update_doc = doc! {};

    // Check if the updated_name is empty or None
    if let Some(name) = updated_name {
        if name.trim().is_empty() {
            return Ok(HttpResponse::BadRequest().body("Name cannot be empty")); // Return error if name is empty
        }
        update_doc.insert("name", name);
    }

    // Check if the updated_age is None or an invalid value (like 0 or negative)
    if let Some(age) = updated_age {
        if age == 0 {
            return Ok(HttpResponse::BadRequest().body("Age must be a positive number")); // Return error if age is invalid
        }
        update_doc.insert("age", age);
    }

    // If neither name nor age is provided, return BadRequest
    if update_doc.is_empty() {
        return Ok(HttpResponse::BadRequest().body("No valid fields to update")); // Return error if no valid fields to update
    }

    // Perform the update
    let result = collection
        .update_one(doc! { "_id": object_id }, doc! { "$set": update_doc }, None)
        .await?;

    // If no document is matched for update, return NotFound
    if result.matched_count == 0 {
        return Ok(HttpResponse::NotFound().body("Document not found"));
    }

    Ok(HttpResponse::Ok().body("Document updated successfully"))
}

// Function to delete a document from the collection
pub async fn delete_document(
    collection: &Collection<Collection1>,
    id: String,
) -> mongodb::error::Result<HttpResponse> {
    // Parse the string ID to ObjectId
    let obj_id = match mongodb::bson::oid::ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid ID format")), // Return BadRequest if ID format is invalid
    };

    let filter = doc! { "_id": obj_id };
    let result = collection.delete_one(filter, None).await?;

    // If the document is deleted successfully, return Ok
    if result.deleted_count == 1 {
        Ok(HttpResponse::Ok().body("Document deleted successfully"))
    } else {
        Ok(HttpResponse::NotFound().body("Document not found")) // If document is not found, return NotFound
    }
}
