use std::fmt::Display;

use blake3::Hasher;
use chrono::{DateTime, Utc};

pub trait HashSaltDebug {
    fn hash_salt_debug<S: Display>(&self, salt: S) -> String;
}

impl<T: std::fmt::Debug> HashSaltDebug for T {
    fn hash_salt_debug<S: Display>(&self, salt: S) -> String {
        let mut hasher = Hasher::new();
        let _ = hasher.update(format!("{}{:?}", salt, self).as_bytes());
        hasher.finalize().to_string()
    }
}

pub trait HashDebug {
    fn hash_debug(&self) -> String;
}

impl<T: std::fmt::Debug> HashDebug for T {
    fn hash_debug(&self) -> String {
        let mut hasher = Hasher::new();
        let _ = hasher.update(format!("{:?}", self).as_bytes());
        hasher.finalize().to_string()
    }
}

pub trait Expired {
    fn expired(&self) -> bool;
}

impl Expired for DateTime<Utc> {
    fn expired(&self) -> bool {
        (self.timestamp() - Utc::now().timestamp()).is_negative()
    }
}

#[macro_export]
macro_rules! impl_error_wrapper {
    ($wrapper_type:ident, $inner_type:ty) => {
        #[derive(Error, Debug)]
        pub struct $wrapper_type(#[from] pub $inner_type);

        impl fmt::Display for $wrapper_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
