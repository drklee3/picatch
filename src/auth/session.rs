use crate::error::{Error, Result};
use crate::model::pool::Pool;
use crate::model::store::{Session, User};
use actix_identity::Identity;
use actix_web::web;
use diesel::prelude::*;

/// Responds error if user already logged in, else does nothing.  If a session is
/// invalid, the invalid session id will be removed from the user's browser
pub async fn verify_not_logged_in(id: &Identity, pool: web::Data<Pool>) -> Result<()> {
    if let Some(session_id) = id.identity() {
        let user = web::block(move || get_session_user(&session_id, pool)).await;

        if user.is_ok() {
            return Err(Error::BadRequest("Already logged in".into()));
        } else {
            // Remove invalid session id
            id.forget();
        }
    }

    Ok(())
}

pub fn get_session_user(current_session_id: &str, pool: web::Data<Pool>) -> Result<User> {
    use crate::schema::sessions::dsl::{id as session_id, sessions};
    use crate::schema::users::dsl::{id as user_id, users};

    let conn = &pool.get().unwrap();

    sessions
        .filter(session_id.eq(current_session_id))
        .first::<Session>(conn)
        .and_then(|session| {
            users
                .filter(user_id.eq(session.user_id))
                .first::<User>(conn)
        })
        .map_err(|_| Error::BadRequest("Invalid session".into()))
}
