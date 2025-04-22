
// Import the mock WebAssembly implementation
import * as wasm from '../mock_wasm.js';

// Import the WebAssembly memory
const memory = wasm.memory;

// Import the send_to_edge function from ws_bridge.js
import { send_to_edge, execute_rust_run } from '../ws_bridge.js';

// Export the WebAssembly functions
export function execute_attack() {
    return wasm.execute_attack();
}

export function log_error(message) {
    return wasm.log_error(message);
}

export function get_execution_output() {
    return wasm.get_execution_output ? wasm.get_execution_output() : '';
}

export function is_execution_in_progress() {
    return wasm.is_execution_in_progress ? wasm.is_execution_in_progress() : false;
}

// Initialize the WebAssembly module
export default function init() {
    console.log("Initializing mock WebAssembly module...");
    return Promise.resolve();
}
