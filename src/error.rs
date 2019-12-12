use actix_web::{HttpResponse, ResponseError};
use image::ImageError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

/// Common result type
pub type Result<T> = StdResult<T, Error>;

/// Common error type to hold errors from other crates
#[derive(Debug)]
pub enum Error {
    /// A `image` crate error
    Image(ImageError),
    /// A `std::io` crate error
    Io(IoError),
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Error {
        Error::Image(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Image(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Image(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Image(_) => HttpResponse::BadRequest().finish(),
            Error::Io(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
