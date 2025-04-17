import os
import shutil
import subprocess
import sys

def run_command(command):
    print(f"Running: {command}")
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
        return False
    print(result.stdout)
    return True

def create_pkg_directory():
    # Create pkg directory if it doesn't exist
    if not os.path.exists("pkg"):
        os.makedirs("pkg")
    
    # Create the JavaScript bindings using the mock implementation
    js_bindings = """
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
"""
    
    with open("pkg/wasm_proxy_app.js", "w") as f:
        f.write(js_bindings)
    
    return True

def main():
    # Build the WebAssembly module
    if not run_command("cargo build"):
        print("Failed to build the WebAssembly module.")
        return
    
    # Create the pkg directory and copy the files
    if not create_pkg_directory():
        print("Failed to create the pkg directory.")
        return
    
    # Print instructions for starting the HTTP server
    print("\nSetup completed successfully!")
    print("To start the HTTP server, run the following command in a new terminal:")
    print("python -m http.server 8080")
    print("\nThen open your browser and navigate to:")
    print("http://localhost:8080/")

if __name__ == "__main__":
    main()
