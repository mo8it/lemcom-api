use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct User {
    pub key: String,
}