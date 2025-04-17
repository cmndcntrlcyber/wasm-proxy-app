// ws_bridge.js
let socket = null;

export function send_to_edge(json) {
    if (!socket || socket.readyState === WebSocket.CLOSED) {
        socket = new WebSocket("ws://localhost:9222/devtools/page/YOUR-TARGET-ID"); // Replace with actual target ID
        socket.onopen = () => {
            socket.send(json);
        };
        socket.onerror = err => {
            console.error("WebSocket error:", err);
        };
    } else if (socket.readyState === WebSocket.OPEN) {
        socket.send(json);
    } else {
        socket.onopen = () => socket.send(json);
    }
}
