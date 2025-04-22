#!/usr/bin/env python3
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
                    output += "\nSTDERR:\n" + result.stderr
                self.wfile.write(output.encode('utf-8'))
            else:
                error_message = f"Error executing binary (code {result.returncode}):\n{result.stderr}"
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
