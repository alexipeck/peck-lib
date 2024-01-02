use thiserror::Error;

pub mod r#trait;

#[derive(Error, Debug)]
pub enum Error {}
