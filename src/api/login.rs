use actix_identity::Identity;
use actix_web::{post, HttpResponse};

#[post("/login")]
async fn post_login(id: Identity) -> HttpResponse {
    id.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}
