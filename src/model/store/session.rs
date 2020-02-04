use super::User;
use crate::schema::sessions;
use actix_web::HttpRequest;
use base64;
use chrono;
use ipnetwork::IpNetwork;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Associations, Queryable, Identifiable, Insertable)]
#[belongs_to(User)]
pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<String>,
    pub last_activity: chrono::NaiveDateTime,
}

impl Session {
    pub fn new(
        user_id: i32,
        ip_address: Option<IpNetwork>,
        user_agent: Option<String>,
        last_activity: chrono::NaiveDateTime,
    ) -> Session {
        Session {
            id: generate_session_id(),
            user_id,
            ip_address,
            user_agent,
            last_activity,
        }
    }

    pub fn new_from_httprequest(user: &User, req: &HttpRequest) -> Session {
        let ip_addr = req.peer_addr().map(|sock_addr| sock_addr.ip());

        let ip_network = ip_addr.and_then(|ip_addr| {
            let prefix = if ip_addr.is_ipv4() { 32 } else { 128 };

            // Postgres uses INET which is networks + addresses
            // Need to use the netmask of 32 or 128 for ipv4 / ipv6 respectively
            // to represent a single host
            IpNetwork::new(ip_addr, prefix).ok()
        });

        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|x| x.to_str().ok())
            .map(String::from);

        Session::new(
            user.id,
            ip_network,
            user_agent,
            chrono::Utc::now().naive_utc(),
        )
    }
}

fn generate_session_id() -> String {
    let mut rng = ChaCha12Rng::from_entropy();

    let mut buf = [0; 32];
    rng.fill_bytes(&mut buf);

    base64::encode(&buf)
}
