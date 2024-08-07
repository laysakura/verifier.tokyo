use wasm_bindgen::prelude::*;

// Function to be called from TypeScript
#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from Rust!".to_string()
}
