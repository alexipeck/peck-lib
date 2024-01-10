use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DuplicateUIDInserted({0})")]
    DuplicateUIDInserted(Uuid),
    #[error("DuplicateUIDsInserted({:?})", 0)]
    DuplicateUIDsInserted(Vec<Uuid>),
}
