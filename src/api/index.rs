use actix_identity::Identity;
use actix_web::get;

#[get("/")]
pub async fn get_index(id: Identity) -> String {
    format!(
        "Hello, your session id is {}",
        id.identity().unwrap_or_else(|| "N/A".to_owned())
    )
}
