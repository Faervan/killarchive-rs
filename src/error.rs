use std::fmt::Display;

use actix_web::{error::InternalError, http::StatusCode};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    PsqlError(tokio_postgres::Error),
    IoError(std::io::Error),
    MpscRecvError,
    TeraError(tera::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Self {
        Self::PsqlError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<tera::Error> for Error {
    fn from(error: tera::Error) -> Self {
        Self::TeraError(error)
    }
}

impl Into<actix_web::Error> for Error {
    fn into(self) -> actix_web::Error {
        actix_web::Error::from(InternalError::new(self, StatusCode::INTERNAL_SERVER_ERROR))
    }
}
