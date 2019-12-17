use crate::schema::users;
use chrono;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Associations, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}
