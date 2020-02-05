use actix_identity::Identity;
use actix_web::{post, HttpResponse};

#[post("/logout")]
async fn post_logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}
