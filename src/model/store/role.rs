use crate::schema::roles;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub admin: bool,
    pub download: bool,
    pub edit: bool,
    pub upload: bool,
    pub view: bool,
}
