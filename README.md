# WASM Proxy Application

A WebAssembly application that executes a .bin file as a child process and proxies the output traffic using HTTPS.

## Features

- Execute binary files from a web interface
- Process binary output in WebAssembly
- Proxy the output via secure HTTPS connection
- Simple and intuitive user interface

## Prerequisites

- [Node.js](https://nodejs.org/) (v14 or later)
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [OpenSSL](https://www.openssl.org/) (for generating SSL certificates)

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd wasm-proxy-app
```

2. Install Node.js dependencies:

```bash
npm install
```

3. Generate SSL certificates for HTTPS:

```bash
mkdir -p certs
cd certs
openssl genrsa -out key.pem 2048
openssl req -new -key key.pem -out csr.pem
openssl x509 -req -days 365 -in csr.pem -signkey key.pem -out cert.pem
rm csr.pem
cd ..
```

## Building the Application

1. Build the Rust WASM module:

```bash
wasm-pack build --target web
```

2. Bundle the application with webpack:

```bash
npx webpack --config webpack.config.js
```

Alternatively, you can use the npm script:

```bash
npm run build
```

## Running the Application

Start the server:

```bash
npm start
```

This will start:
- An HTTP server on port 3000 for the main application
- An HTTPS server on port 3001 for the proxy

Open your browser and navigate to:

```
http://localhost:3000
```

## Usage

1. Enter the path to the binary file you want to execute
2. Add any arguments required by the binary (one per line)
3. Specify the HTTPS proxy URL (default is https://localhost:3001/proxy)
4. Click "Execute and Proxy"
5. View the results in the output section

## Security Considerations

- This application executes binaries on the server, which can be a security risk if not properly controlled
- In a production environment, you should:
  - Implement proper authentication and authorization
  - Restrict which binaries can be executed
  - Use proper SSL certificates instead of self-signed ones
  - Add input validation and sanitization

## Architecture

The application consists of:

1. **Rust WASM Module**: Handles the communication between the browser and the server
2. **Node.js Server**: Executes the binary files and serves the web application
3. **HTTPS Proxy Server**: Securely transmits the binary output

## License

MIT
