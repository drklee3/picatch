use crate::auth::crypto::hash_password;
use crate::auth::session;
use crate::error::{Error, Result};
use crate::model::auth::AuthData;
use crate::model::pool::Pool;
use crate::model::store::{NewUser, Session, User};
use actix_identity::Identity;
use actix_web::{error::BlockingError, post, web, HttpRequest, HttpResponse};
use chrono::Utc;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{insert_into, RunQueryDsl};

#[post("/register")]
async fn post_register(
    auth_data: web::Form<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    session::verify_not_logged_in(&id, pool.clone()).await?;

    web::block(move || register_query(auth_data.into_inner(), pool)).await?;

    Ok(HttpResponse::Ok().finish())
}

fn register_query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<User> {
    use crate::schema::users::dsl::users;
    let conn = &pool.get().unwrap();

    // Also check if username already taken when filling in the form?

    let new_user = NewUser {
        username: auth_data.username,
        hash: hash_password(&auth_data.password)?,
        created_at: Utc::now().naive_utc(),
    };

    let res = insert_into(users).values(new_user).get_result(conn);

    res.map_err(|e| match e {
        DatabaseError(db_e, _) => match db_e {
            DatabaseErrorKind::UniqueViolation => {
                Error::BadRequest("Username already taken".into())
            }
            _ => Error::InternalServerError,
        },
        _ => Error::InternalServerError,
    })
}
