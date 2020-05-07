use actix_web::{error::BlockingError, HttpResponse, ResponseError};
use fern::InitError as FernError;
use image::ImageError;
use log::SetLoggerError;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::Error as FmtError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSeError;

/// Common result type
pub type Result<T> = StdResult<T, Error>;

/// Common error type to hold errors from other crates
#[derive(Debug)]
pub enum Error {
    /// `fern` error
    Fern(FernError),
    /// `std::fmt` error
    Fmt(FmtError),
    /// `image` error
    Image(ImageError),
    /// `std::io` error
    Io(IoError),
    /// `log` set_logger error
    SetLogger(SetLoggerError),
    /// `toml` deserialize error
    TomlDe(TomlDeError),
    /// `toml` serialize error
    TomlSe(TomlSeError),
    /// Custom errors to give http responses
    Unauthorized,
    BadRequest(String),
    InternalServerError,
}
impl From<FernError> for Error {
    fn from(err: FernError) -> Error {
        Error::Fern(err)
    }
}

impl From<FmtError> for Error {
    fn from(err: FmtError) -> Error {
        Error::Fmt(err)
    }
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

impl From<SetLoggerError> for Error {
    fn from(err: SetLoggerError) -> Error {
        Error::SetLogger(err)
    }
}

impl From<TomlDeError> for Error {
    fn from(err: TomlDeError) -> Error {
        Error::TomlDe(err)
    }
}

impl From<TomlSeError> for Error {
    fn from(err: TomlSeError) -> Error {
        Error::TomlSe(err)
    }
}

// Actix web::block response, we can just use ? to handle blocking calls now
// like so:  web::block(...).await?
// Instead of having to do a match for every blocking call
impl From<BlockingError<Error>> for Error {
    fn from(error: BlockingError<Error>) -> Error {
        match error {
            BlockingError::Error(err) => err,
            BlockingError::Canceled => Error::InternalServerError,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Fern(ref inner) => inner.fmt(f),
            Error::Fmt(ref inner) => inner.fmt(f),
            Error::Image(ref inner) => inner.fmt(f),
            Error::SetLogger(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            Error::TomlDe(ref inner) => inner.fmt(f),
            Error::TomlSe(ref inner) => inner.fmt(f),
            Error::InternalServerError => write!(f, "Internal Server Error"),
            Error::BadRequest(ref inner) => write!(f, "BadRequest: {}", inner),
            Error::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Fern(ref inner) => inner.description(),
            Error::Fmt(ref inner) => inner.description(),
            Error::Image(ref inner) => inner.description(),
            Error::SetLogger(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::TomlDe(ref inner) => inner.description(),
            Error::TomlSe(ref inner) => inner.description(),
            Error::BadRequest(ref inner) => inner,
            Error::InternalServerError => "Internal Server Error",
            Error::Unauthorized => "Unauthorized",
            _ => "uhh",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonErrorResponse {
    pub status: String,
    pub message: String,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Image(_) => HttpResponse::NotFound().finish(),
            Error::Io(_) => HttpResponse::InternalServerError().finish(),
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json(JsonErrorResponse {
                    status: "InternalServerError".into(),
                    message: "Internal Server Error, Please try later".into(),
                })
            }
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(JsonErrorResponse {
                status: "BadRequest".into(),
                message: message.into(),
            }),
            Error::Unauthorized => HttpResponse::Unauthorized().json(JsonErrorResponse {
                status: "Unauthorized".into(),
                message: "Unauthorized".into(),
            }),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
