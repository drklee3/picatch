use crate::schema::permissions;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable)]
pub struct Permission {
    pub id: i32,
    pub name: String,
}
