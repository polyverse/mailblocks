use sha2::{Digest, Sha256};

pub fn encrypt(msg: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(msg);
    let hash = hasher.result();
    let output = format!("{:x}", hash);
    return String::from(output);
}
