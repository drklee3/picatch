use crate::schema::roles;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable)]
pub struct Role {
    pub id: i32,
    pub name: String,
}