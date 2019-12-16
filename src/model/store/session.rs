use super::User;
use crate::schema::sessions;
use chrono;
use ipnetwork::IpNetwork;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Associations, Queryable, Identifiable)]
#[belongs_to(User)]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub ip_address: IpNetwork,
    pub user_agent: String,
    pub last_activity: chrono::NaiveDateTime,
}
