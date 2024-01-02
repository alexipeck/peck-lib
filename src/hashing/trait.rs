use std::fmt::Display;

use blake3::Hasher;

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
