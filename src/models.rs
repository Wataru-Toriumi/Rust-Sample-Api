use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
}
