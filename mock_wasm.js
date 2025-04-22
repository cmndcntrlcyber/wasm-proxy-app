// This is a mock implementation of the WebAssembly module
// It simulates the behavior of the actual WebAssembly module for testing purposes
// Updated to support rust-run execution proxying

// Mock WebAssembly memory
export const memory = {};

// Execution state
let executionInProgress = false;
let executionOutput = "";

// Mock execute_attack function
export function execute_attack() {
    console.log("Starting execution of rust-run application...");
    
    // Set execution state
    executionInProgress = true;
    executionOutput = "";
    
    // Simulate a delay to mimic the actual rust-run execution
    return new Promise(resolve => {
        setTimeout(() => {
            // Add some simulated output
            executionOutput = `
Application starting
Security checks passed
Dictionary retrieval successful
Payload retrieval successful
Payload decoding successful
Executing 128 bytes of shellcode
Shellcode execution completed
Application completed successfully
`;
            
            console.log("rust-run execution completed");
            executionInProgress = false;
            
            // Dispatch a custom event for the UI
            const outputEvent = new CustomEvent('rust-run-output', { 
                detail: { output: "Application completed successfully" }
            });
            window.dispatchEvent(outputEvent);
            
            resolve("rust-run execution completed successfully");
        }, 2000);
    });
}

// Mock log_error function
export function log_error(message) {
    console.error("WASM Error:", message);
}

// Mock get_execution_output function
export function get_execution_output() {
    return executionOutput;
}

// Mock is_execution_in_progress function
export function is_execution_in_progress() {
    return executionInProgress;
}
