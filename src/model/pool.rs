use diesel::{r2d2, r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
