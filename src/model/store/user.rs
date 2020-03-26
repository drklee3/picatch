use crate::schema::users;
use chrono;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Associations, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Struct used for only inserting, does not require the use of id field as it
/// is a SERIAL pg data type which is auto assigned.
#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}
