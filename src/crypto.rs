use sha2::{Sha256, Digest};

pub fn encrypt(msg: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(msg);
    let hash = hasher.result();
    let output = format!("{:x}", hash);
    return String::from(output);
}