use thiserror::Error;

use crate::impl_error_wrapper;

impl_error_wrapper!(SerdeError, serde_json::error::Error);
impl_error_wrapper!(RSAError, rsa::errors::Error);

#[derive(Error, Debug)]
pub enum Error {
    #[error("SerialiseLoginCredentials({0})")]
    SerialiseLoginCredentials(SerdeError),
    #[error("EncryptLoginCredentials({0})")]
    EncryptLoginCredentials(RSAError),
}
