pub mod utils;
pub mod crypto;

use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Mailblock struct for JSON output
#[derive(Serialize)]
struct Header {
    title: String,
    version: String
}
#[derive(Serialize)]
struct Mailblock {
    header: Header,
    index: i32,
    total: i32,
    hash: String
}

#[wasm_bindgen]
pub fn forge_mailblock(message: &str, index: i32, total: i32) -> JsValue {
    let hash = crypto::encrypt(message);
    let mailblock = Mailblock {
        header: Header {
            title: String::from("mailblock"),
            version: String::from("1.0")
        },
        index: index,
        total: total,
        hash: hash
    };
    match JsValue::from_serde(&mailblock) {
        Ok(json) => json,
        Err(_) => JsValue::NULL
    }
}