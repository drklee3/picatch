use super::User;
use crate::schema::sessions;
use base64;
use chrono;
use ipnetwork::IpNetwork;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Associations, Queryable, Identifiable)]
#[belongs_to(User)]
pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub ip_address: IpNetwork,
    pub user_agent: String,
    pub last_activity: chrono::NaiveDateTime,
}

impl Session {
    pub fn new<I: Into<IpNetwork>>(
        user_id: i32,
        ip_address: I,
        user_agent: String,
        last_activity: chrono::NaiveDateTime,
    ) -> Session {
        Session {
            id: generate_session_id(),
            user_id,
            ip_address: ip_address.into(),
            user_agent,
            last_activity,
        }
    }
}

fn generate_session_id() -> String {
    let mut rng = ChaCha12Rng::from_entropy();

    let mut buf = [0; 32];
    rng.fill_bytes(&mut buf);

    base64::encode(&buf)
}
