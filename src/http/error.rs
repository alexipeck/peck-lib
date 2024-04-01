use crate::impl_error_wrapper;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::StatusCode;
#[cfg(target_arch = "wasm32")]
use reqwest_wasm::StatusCode;
use thiserror::Error;

impl_error_wrapper!(ReqwestError, reqwest::Error);
impl_error_wrapper!(SerdeJsonError, serde_json::error::Error);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Send({0})")]
    Send(ReqwestError),
    #[error("ConvertResponseToText({0})")]
    ConvertResponseToText(ReqwestError),
    #[error("DeserializeTypeFromText({0})")]
    DeserializeTypeFromText(SerdeJsonError),
    #[error("InvalidResponse({0}: {1})")]
    InvalidResponse(StatusCode, String),
}

/* impl From<ReqwestError> for crate::error::Error {
    fn from(value: ReqwestError) -> Self {
        crate::error::Error::Reqwest(value)
    }
}

impl From<SerdeJsonError> for crate::error::Error {
    fn from(value: SerdeJsonError) -> Self {
        crate::error::Error::SerdeJson(value)
    }
} */
