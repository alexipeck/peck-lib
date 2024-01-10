pub mod r#trait;

use crate::impl_error_wrapper;

impl_error_wrapper!(ReqwestError, reqwest::Error);
impl_error_wrapper!(SerdeJsonError, serde_json::error::Error);
