
// Import the mock WebAssembly implementation
import * as wasm from '../mock_wasm.js';

// Import the WebAssembly memory
const memory = wasm.memory;

// Import the send_to_edge function from ws_bridge.js
import { send_to_edge } from '../ws_bridge.js';

// Export the WebAssembly functions
export function execute_attack() {
    return wasm.execute_attack();
}

export function log_error(message) {
    return wasm.log_error(message);
}

// Initialize the WebAssembly module
export default function init() {
    console.log("Initializing mock WebAssembly module...");
    return Promise.resolve();
}
