use super::{Role, User};
use crate::schema::user_roles;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Associations, Queryable, Identifiable)]
#[primary_key(user_id, role_id)]
#[belongs_to(Role)]
#[belongs_to(User)]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
}
