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

def check_rust_run_executable():
    """Check if the rust-run executable exists"""
    rust_run_path = os.path.normpath(os.path.join("..", "rust-run", "target", "release", "rust-run.exe"))
    
    if not os.path.exists(rust_run_path):
        print(f"Warning: rust-run executable not found at {rust_run_path}")
        print("Building rust-run project...")
        
        rust_run_dir = os.path.normpath(os.path.join("..", "rust-run"))
        if os.path.exists(rust_run_dir):
            # Build the rust-run project
            current_dir = os.getcwd()
            os.chdir(rust_run_dir)
            success = run_command("cargo build --release")
            os.chdir(current_dir)
            
            if success:
                print("Successfully built rust-run executable.")
            else:
                print("Failed to build rust-run executable.")
                return False
        else:
            print(f"Error: rust-run directory not found at {rust_run_dir}")
            return False
    
    print(f"rust-run executable found at {rust_run_path}")
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
"""
    
    with open("pkg/wasm_proxy_app.js", "w") as f:
        f.write(js_bindings)
    
    return True

def create_server_proxy_files():
    """Create Python proxy server files to handle binary execution"""
    
    # Create a simple Python proxy server script
    server_script = """#!/usr/bin/env python3
import http.server
import socketserver
import urllib.parse
import subprocess
import json
import os
from http import HTTPStatus

PORT = 3000

class ProxyHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        parsed_path = urllib.parse.urlparse(self.path)
        query_params = urllib.parse.parse_qs(parsed_path.query)
        
        # Handle binary execution request
        if parsed_path.path == '/execute-binary':
            if 'path' in query_params:
                binary_path = query_params['path'][0]
                
                print(f"Executing binary: {binary_path}")
                self.execute_binary(binary_path)
            else:
                self.send_error(HTTPStatus.BAD_REQUEST, "Missing 'path' parameter")
        else:
            self.send_error(HTTPStatus.NOT_FOUND, "Path not found")
    
    def execute_binary(self, binary_path):
        try:
            # Make sure the path exists
            if not os.path.exists(binary_path):
                self.send_error(HTTPStatus.NOT_FOUND, f"Binary not found: {binary_path}")
                return
            
            # Execute the binary
            result = subprocess.run(
                binary_path, 
                shell=True, 
                capture_output=True, 
                text=True
            )
            
            # Set headers and status code
            self.send_response(HTTPStatus.OK)
            self.send_header('Content-Type', 'text/plain')
            self.send_header('Access-Control-Allow-Origin', '*')  # For CORS
            self.end_headers()
            
            # Send the output
            if result.returncode == 0:
                output = result.stdout
                if result.stderr:
                    output += "\\nSTDERR:\\n" + result.stderr
                self.wfile.write(output.encode('utf-8'))
            else:
                error_message = f"Error executing binary (code {result.returncode}):\\n{result.stderr}"
                self.wfile.write(error_message.encode('utf-8'))
                
        except Exception as e:
            self.send_error(HTTPStatus.INTERNAL_SERVER_ERROR, str(e))

if __name__ == "__main__":
    handler = ProxyHandler
    
    with socketserver.TCPServer(("", PORT), handler) as httpd:
        print(f"Proxy server running at http://localhost:{PORT}")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("Server stopped by user")
            httpd.server_close()
"""
    
    with open("proxy_server.py", "w") as f:
        f.write(server_script)
    
    print("Created Python proxy server script.")
    return True

def main():
    # Check if rust-run executable exists
    if not check_rust_run_executable():
        print("Warning: rust-run executable not available. The application may not function correctly.")
    
    # Build the WebAssembly module
    if not run_command("cargo build"):
        print("Failed to build the WebAssembly module.")
        return
    
    # Create the pkg directory and copy the files
    if not create_pkg_directory():
        print("Failed to create the pkg directory.")
        return
    
    # Create server proxy files
    if not create_server_proxy_files():
        print("Failed to create server proxy files.")
        return
    
    # Print instructions for starting the HTTP server
    print("\nSetup completed successfully!")
    print("To start the proxy server, run the following command in a new terminal:")
    print("python proxy_server.py")
    print("\nTo start the HTTP server, run the following command in another terminal:")
    print("python -m http.server 8080")
    print("\nThen open your browser and navigate to:")
    print("http://localhost:8080/")

if __name__ == "__main__":
    main()
