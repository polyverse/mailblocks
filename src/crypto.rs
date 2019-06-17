use sha2::{Sha256, Digest};

pub fn encrypt(msg: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(msg);
    // hasher.input("String data");
    let hash = hasher.result();
    println!("Hashed result: {:x}", hash);
    let output = format!("{:x}", hash);
    return String::from(output);
}