extern crate mailblocks;
use mailblocks::crypto;

#[test]
fn test_hash() {
    crypto::encrypt("Hello world");
}