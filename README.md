# WASM Proxy Application for rust-run

A WebAssembly application that executes the `../rust-run/` binary file and proxies its output through a web interface.

## Features

- Execute the rust-run binary from a web interface
- Proxy and display the command output in real-time
- Track execution status and handle errors
- Simple and intuitive user interface

## Prerequisites

- [Node.js](https://nodejs.org/) (v14 or later)
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python 3.x (for running the simple HTTP server)

## Project Structure

- **src/lib.rs**: WASM module that interfaces with the rust-run binary
- **ws_bridge.js**: WebSocket bridge for communication with the proxy server
- **proxy_server.py**: Python-based server that executes the rust-run binary
- **index.html & index.js**: Web UI for user interaction
- **mock_wasm.js**: Mock implementation for testing without the actual WASM module
- **build.py**: Script to build and set up the project

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd wasm-proxy-app
```

2. Make sure the rust-run project is properly set up in the parent directory:

```bash
# Check if rust-run project exists
ls ../rust-run

# If needed, build the rust-run project
cd ../rust-run
cargo build --release
cd ../wasm-proxy-app
```

## Building the Application

Run the build script which will:
- Check for the rust-run executable
- Build the Rust WASM module
- Create the proxy server script
- Set up the mock implementation for testing

```bash
python build.py
```

## Running the Application

1. Start the proxy server:

```bash
python proxy_server.py
```

2. In a separate terminal, start the HTTP server:

```bash
python -m http.server 8080
```

3. Open your browser and navigate to:

```
http://localhost:8080
```

## Usage

1. Open the web application in your browser
2. Click "Execute rust-run" to start the process
3. View the real-time output in the output section
4. Check the status indicator for execution progress

## How It Works

1. The web application initiates execution through the WASM module
2. The WASM module communicates with the proxy server via a WebSocket bridge
3. The proxy server executes the rust-run binary
4. Output from the binary is captured and streamed back to the web UI
5. Real-time updates are displayed to the user

## Development

For development and testing without building the WASM module:
- The project includes a mock implementation (mock_wasm.js)
- This allows for testing the UI and flow without executing the actual binary

## Troubleshooting

- **Proxy Server Issues**: Make sure the Python proxy server is running on port 3000
- **WASM Module Errors**: Check the browser console for detailed error messages
- **rust-run Binary Issues**: Ensure the binary exists and has execution permissions

## License

MIT
