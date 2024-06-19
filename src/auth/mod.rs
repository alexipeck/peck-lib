pub mod error;
pub mod token_pair;
pub mod user_login;

pub fn str_to_two_fa(input: &str) -> Option<[u8; 6]> {
    let bytes = input.as_bytes();
    if bytes.len() != 6 {
        return None;
    }
    let mut array = [0u8; 6];
    array.copy_from_slice(bytes);
    Some(array)
}
