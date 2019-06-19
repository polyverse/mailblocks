pub mod utils;
pub mod crypto;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let hash = hash_message();
    let preamble = "Hashed msg: ";
    let message = format!("{}{}", preamble, hash);
    alert(&message);
}

#[wasm_bindgen]
pub fn hash_message() -> String{
    let email = String::from("Email stand in string");
    let hash = crypto::encrypt(&email);
    hash
}