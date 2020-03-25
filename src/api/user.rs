use crate::error::Result;
use crate::model::pool::Pool;
use actix_web::{get, web, HttpResponse};
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
