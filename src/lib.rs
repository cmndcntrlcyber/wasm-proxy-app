use wasm_bindgen::prelude::*;
use web_sys::console;
use console_error_panic_hook;
use js_sys::{Promise, Function};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Static flag to track if execution is in progress
static EXECUTION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static EXECUTION_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

#[wasm_bindgen(module = "ws_bridge.js")]
extern "C" {
    fn send_to_edge(json: &str);
    fn execute_rust_run(path: &str, callback: &Function) -> i32;
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

/// Execute the rust-run binary and proxy its output
#[wasm_bindgen]
pub async fn execute_attack() -> Result<JsValue, JsValue> {
    // Prevent multiple executions running simultaneously
    if EXECUTION_IN_PROGRESS.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err(JsValue::from_str("Execution already in progress"));
    }

    console::log_1(&"Starting execution of rust-run application...".into());
    
    // Clear any previous execution output
    if let Ok(mut output) = EXECUTION_OUTPUT.lock() {
        *output = String::new();
    }
    
    // Path to the rust-run executable relative to the server root
    let rust_run_path = "../rust-run/target/release/rust-run.exe";
    
    // Log the execution attempt
    console::log_1(&format!("Executing rust-run binary at: {}", rust_run_path).into());
    
    // Create a Promise to handle the async execution
    let promise = Promise::new(&mut |resolve: Function, _reject: Function| {
        // Create callback function to process execution results
        let callback = Closure::once_into_js(move |result: JsValue| {
            EXECUTION_IN_PROGRESS.store(false, Ordering::SeqCst);
            
            // Store the result in our output buffer
            if let Some(result_str) = result.as_string() {
                if let Ok(mut output) = EXECUTION_OUTPUT.lock() {
                    *output = result_str;
                }
            }
            
            let _ = resolve.call1(&JsValue::NULL, &result);
        });
        
        // Execute the rust-run binary through our WebSocket bridge
        execute_rust_run(rust_run_path, &callback.into());
    });
    
    // Wait for the promise to resolve
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    
    // Return the execution result
    Ok(result)
}

/// Get the latest output from the rust-run execution
#[wasm_bindgen]
pub fn get_execution_output() -> String {
    EXECUTION_OUTPUT.lock()
        .map(|guard| guard.clone())
        .unwrap_or_else(|_| String::new())
}

/// Check if execution is currently in progress
#[wasm_bindgen]
pub fn is_execution_in_progress() -> bool {
    EXECUTION_IN_PROGRESS.load(Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn log_error(msg: String) {
    console::error_1(&JsValue::from_str(&msg));
}
