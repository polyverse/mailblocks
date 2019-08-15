pub mod crypto;
pub mod utils;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Mailblock struct for JSON output
// #[derive(Serialize)]
// struct Header {
//     title: String,
//     version: String,
// }
#[derive(Serialize)]
struct Mailblock {
    title: String,
    version: String,
    stamp: String,
    recipient: String,
    hash: String,
}

#[wasm_bindgen]
pub fn generate_mailblock(message: &str, stamp: String, recipient: String) -> JsValue {
    let magic = String::from("init-magic");
    let hash;
    match message == magic {
        true => hash = magic, 
        _ => hash = crypto::encrypt(message)
    }
    let mailblock = Mailblock {
        title: String::from("mailblock"),
        version: String::from("1.0"),
        stamp: stamp,
        recipient: recipient,
        hash: hash,
    };
    match JsValue::from_serde(&mailblock) {
        Ok(json) => json,
        Err(_) => JsValue::NULL,
    }
}
