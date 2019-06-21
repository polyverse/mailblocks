extern crate mailblocks;
use mailblocks::crypto;
use mailblocks::utils;

#[test]
fn test_hash() {
    let email_path = "./tests/sample_emails/test.txt";
    let email = utils::read_file(email_path);
    crypto::encrypt(&email);
}

#[test]
fn test_read() {
    let email_path = "./tests/sample_emails/test.txt";
    utils::read_file(email_path);
}
