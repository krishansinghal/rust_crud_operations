use serde::{Deserialize, Serialize};
// use mongodb::bson::DateTime; // Uncomment if you want to use the DateTime field for created_at

#[derive(Clone, Debug, Serialize, Deserialize)]  // Enable serde serialization and deserialization
pub struct Collection1 {
    pub name: String,  // The name of the person or entity
    pub age: u32,      // The age of the person or entity
    pub game: String,  // The game associated with the person or entity
    // pub created_at: Option<DateTime>,  // Uncomment this line if you want to track the creation date of the document
}
