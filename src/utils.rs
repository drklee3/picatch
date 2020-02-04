use crate::error::{Error, Result};
use actix_web::{error::BlockingError, web};

async fn run_blocking<F, I>(f: F) -> Result<I>
where
    F: FnOnce() -> Result<I> + Send + 'static,
    I: Send + 'static,
{
    let res = web::block(|| f()).await;

    match res {
        Ok(inner) => Ok(inner),
        Err(err) => match err {
            BlockingError::Error(err) => Err(err),
            BlockingError::Canceled => Err(Error::InternalServerError),
        },
    }
}
