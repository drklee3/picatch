use crate::auth::crypto::verify;
use crate::error::{Error, Result};
use crate::model::auth::AuthData;
use crate::model::pool::Pool;
use crate::model::store::{Session, User};
use actix_identity::Identity;
use actix_web::{post, web, HttpRequest, HttpResponse};
use diesel::insert_into;
use diesel::prelude::*;

#[post("/login")]
async fn post_login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    if id.identity().is_some() {
        return Err(Error::BadRequest("Already logged in".into()));
    }

    // Verify the user login
    let user = {
        let pool = pool.clone();
        web::block(move || login_query(auth_data.into_inner(), pool)).await?
    };

    // Create a new session for the login
    let session = Session::new_from_httprequest(&user, &req);

    {
        let session = session.clone();
        let pool = pool.clone();
        web::block(move || new_session_query(session, pool)).await?;
    }

    // Save session to cookie
    id.remember(session.id);
    Ok(HttpResponse::Ok().finish())
}

fn login_query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<User> {
    use crate::schema::users::dsl::{username, users};
    let conn = &pool.get().unwrap();

    let mut items = users
        .filter(username.eq(&auth_data.username))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user);
            }
        }
    }

    Err(Error::Unauthorized)
}

fn new_session_query(session: Session, pool: web::Data<Pool>) -> Result<()> {
    use crate::schema::sessions::dsl::sessions;
    let conn = &pool.get().unwrap();

    insert_into(sessions).values(session).execute(conn)?;

    Ok(())
}
