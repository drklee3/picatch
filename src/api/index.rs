use actix_identity::Identity;
use actix_web::{get};

#[get("/")]
pub async fn get_index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}
