use crate::error::Result;
use crate::model::pool::Pool;
use actix_web::{get, web, HttpResponse};
use diesel::{RunQueryDsl};
use diesel::dsl::count;
use diesel::{ExpressionMethods, QueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub exists: bool,
}

#[get("/username_exists")]
pub async fn get_username_exists(
    web::Query(user_data): web::Query<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let exists = web::block(move || username_exists_query(user_data.username, pool)).await?;

    Ok(HttpResponse::Ok().json(Response {exists}))
}

fn username_exists_query(query_username: String, pool: web::Data<Pool>) -> Result<bool> {
    use crate::schema::users::dsl::{users, username};
    let conn = &pool.get().unwrap();

    users
        .select(count(username))
        .filter(username.eq(query_username))
        .first(conn)
        .map(|count: i64| count == 1)
        .map_err(Into::into)
}
