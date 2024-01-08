use thiserror::Error;
use uuid::Uuid;

pub mod r#trait;

#[derive(Error, Debug)]
pub enum UIDAuthorityError {
    #[error("DuplicateUIDInserted({0})")]
    DuplicateUIDInserted(Uuid),
    #[error("DuplicateUIDsInserted({:?})", 0)]
    DuplicateUIDsInserted(Vec<Uuid>),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("UIDAuthority({0})")]
    UIDAuthority(UIDAuthorityError),
}
