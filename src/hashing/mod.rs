use blake3::Hasher;

pub mod r#trait;

///hashes with blake3
pub fn hash_string(data: &str) -> String {
    let mut hasher = Hasher::new();
    let _ = hasher.update(data.as_bytes());
    hasher.finalize().to_string()
}
