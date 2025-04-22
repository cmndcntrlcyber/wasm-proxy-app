// ws_bridge.js - Enhanced to support rust-run execution
let socket = null;
let messageQueue = [];
let execCallbacks = {};
let execCounter = 0;

// Function to send messages to the edge server
export function send_to_edge(json) {
    if (!socket || socket.readyState === WebSocket.CLOSED) {
        socket = new WebSocket("ws://localhost:9222/devtools/page/YOUR-TARGET-ID"); // Replace with actual target ID
        
        socket.onopen = () => {
            console.log("WebSocket connection established");
            // Send any queued messages
            while (messageQueue.length > 0) {
                const msg = messageQueue.shift();
                socket.send(msg);
            }
            
            if (json) {
                socket.send(json);
            }
        };
        
        socket.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                if (data.id && execCallbacks[data.id]) {
                    execCallbacks[data.id](data);
                    delete execCallbacks[data.id];
                }
                
                // Handle rust-run execution output
                if (data.method === "Runtime.consoleAPICalled" && 
                    data.params && 
                    data.params.args && 
                    data.params.args.length > 0) {
                    const output = data.params.args.map(arg => arg.value).join(' ');
                    console.log("Rust-run output:", output);
                    
                    // Dispatch a custom event for the UI to capture
                    const outputEvent = new CustomEvent('rust-run-output', { 
                        detail: { output: output }
                    });
                    window.dispatchEvent(outputEvent);
                }
            } catch (e) {
                console.error("Error processing WebSocket message:", e);
            }
        };
        
        socket.onerror = err => {
            console.error("WebSocket error:", err);
        };
        
        socket.onclose = () => {
            console.log("WebSocket connection closed");
        };
    } else if (socket.readyState === WebSocket.OPEN) {
        socket.send(json);
    } else {
        // Queue the message to be sent when the connection opens
        messageQueue.push(json);
    }
}

// Function to execute the rust-run binary through the proxy
export function execute_rust_run(path, callback) {
    const id = ++execCounter;
    
    if (callback) {
        execCallbacks[id] = callback;
    }
    
    const execCommand = JSON.stringify({
        id: id,
        method: "Runtime.evaluate",
        params: {
            expression: `fetch('/execute-binary?path=${encodeURIComponent(path)}')
                .then(r => r.text())
                .then(data => { console.log('Rust-run execution result:', data); return data; })
                .catch(err => { console.error('Rust-run execution error:', err); throw err; })`
        }
    });
    
    send_to_edge(execCommand);
    return id;
}
