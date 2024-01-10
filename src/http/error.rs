use reqwest::StatusCode;
use thiserror::Error;

use crate::error::{ReqwestError, SerdeJsonError};

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
