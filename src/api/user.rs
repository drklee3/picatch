use crate::error::{Error, Result};
use crate::model::pool::Pool;
use crate::auth::session::get_session_user;
use actix_web::{get, web, HttpResponse};
use actix_identity::Identity;
use diesel::dsl::count;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub username: String,
    pub exists: bool,
}

#[get("/username_exists")]
pub async fn get_username_exists(
    web::Query(user_data): web::Query<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let username = user_data.username.clone();
    let exists = web::block(move || username_exists_query(username, pool)).await?;

    Ok(HttpResponse::Ok().json(Response {
        username: user_data.username,
        exists,
    }))
}

fn username_exists_query(query_username: String, pool: web::Data<Pool>) -> Result<bool> {
    use crate::schema::users::dsl::{username, users};
    let conn = &pool.get().unwrap();

    users
        .select(count(username))
        .filter(username.eq(query_username))
        .first(conn)
        .map(|count: i64| count != 0)
        .map_err(Into::into)
}

#[get("/current_user")]
pub async fn get_current_user(
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let session_id = if let Some(session_id) = id.identity() {
        session_id
    } else {
        return Err(Error::Unauthorized);
    };

    let current_user = web::block(move || get_session_user(&session_id, pool)).await?;

    Ok(HttpResponse::Ok().json(current_user))
}
