pub mod r#trait;

use thiserror::Error;

use crate::impl_error_wrapper;

impl_error_wrapper!(ReqwestError, reqwest::Error);
impl_error_wrapper!(SerdeJsonError, serde_json::error::Error);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest({0})")]
    Reqwest(ReqwestError),
    #[error("SerdeJson({0})")]
    SerdeJson(SerdeJsonError),
    #[error("Smtp({0})")]
    Smtp(crate::smtp::error::Error),
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Error::Reqwest(value)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(value: SerdeJsonError) -> Self {
        Error::SerdeJson(value)
    }
}
