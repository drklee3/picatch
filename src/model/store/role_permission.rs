use super::{Permission, Role};
use crate::schema::role_permissions;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Associations, Queryable, Identifiable)]
#[primary_key(role_id, permission_id)]
#[belongs_to(Permission)]
#[belongs_to(Role)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}
