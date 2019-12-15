use actix_web::{HttpResponse, ResponseError};
use argonautica::Error as ArgonauticaError;
use image::ImageError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSeError;
use std::fmt::Error as FmtError;

/// Common result type
pub type Result<T> = StdResult<T, Error>;

/// Common error type to hold errors from other crates
#[derive(Debug)]
pub enum Error {
    /// A `argonautica` crate error
    Argonautica(ArgonauticaError),
    /// a `std::fmt` error
    Fmt(FmtError),
    /// A `image` crate error
    Image(ImageError),
    /// A `std::io` crate error
    Io(IoError),
    /// A `toml` crate deserialize error
    TomlDe(TomlDeError),
    /// A `toml` crate serialize error
    TomlSe(TomlSeError)
    
}

impl From<ArgonauticaError> for Error {
    fn from(err: ArgonauticaError) -> Error {
        Error::Argonautica(err)
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

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::Argonautica(ref inner) => inner.fmt(f),
            Error::Fmt(ref inner) => inner.fmt(f),
            Error::Image(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            Error::TomlDe(ref inner) => inner.fmt(f),
            Error::TomlSe(ref inner) => inner.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Fmt(ref inner) => inner.description(),
            Error::Image(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            Error::TomlDe(ref inner) => inner.description(),
            Error::TomlSe(ref inner) => inner.description(),
            _ => "uhh"
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Image(_) => HttpResponse::NotFound().finish(),
            Error::Io(_) => HttpResponse::InternalServerError().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
