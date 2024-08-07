use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_json::Value;
use reqwest::Client;

// Function to be called from TypeScript
#[wasm_bindgen]
pub async fn fetch_content() -> Result<JsValue, JsValue> {
    let client = Client::new();
    let res = client.get("https://example.com/")
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let text = res.text()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(JsValue::from_str(&text))
}

#[wasm_bindgen]
pub fn decode_and_verify_vc(vc: &str) -> Result<JsValue, JsValue> {
    let decoded: Value = serde_json::from_str(vc)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    // Here you would add your verification logic
    Ok(JsValue::from_serde(&decoded).unwrap())
}
