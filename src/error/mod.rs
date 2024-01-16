pub mod r#trait;

use crate::impl_error_wrapper;

impl_error_wrapper!(ReqwestError, reqwest::Error);
impl_error_wrapper!(SerdeJsonError, serde_json::error::Error);

/* #[derive(thiserror::Error, Debug)]
pub enum Error {
    //#[cfg(feature = "http")]
    #[error("Reqwest({0})")]
    Reqwest(ReqwestError),
    //#[cfg(feature = "http")]
    #[error("SerdeJson({0})")]
    SerdeJson(SerdeJsonError),
    //#[cfg(feature = "smtp")]
    #[error("Smtp({0})")]
    Smtp(crate::smtp::error::Error),
} */
