// Import the WASM module
// Note: This path will be available after wasm-pack build
import init, { execute_and_proxy, execute_attack, log_error } from './pkg/wasm_proxy_app.js';

// Initialize the WASM module
async function initWasm() {
    try {
        await init();
        console.log('WASM module initialized successfully');
        document.getElementById('status').textContent = 'WASM module loaded successfully';
    } catch (error) {
        console.error('Failed to initialize WASM module:', error);
        document.getElementById('status').textContent = 'Failed to load WASM module: ' + error.message;
    }
}

// Execute the binary and proxy its output
async function executeBinaryAndProxy() {
    try {
        // Update status
        const statusElement = document.getElementById('status');
        statusElement.textContent = 'Executing attack sequence...';
        
        // Clear previous output
        document.getElementById('output').textContent = '';
        
        // Run our attack sequence
        console.log('Starting attack sequence');
        
        // Call the WASM attack function
        const result = await execute_attack();
        
        // Display the result
        const outputElement = document.getElementById('output');
        
        if (typeof result === 'object') {
            // If result is an object, stringify it
            outputElement.textContent = JSON.stringify(result, null, 2);
        } else {
            // Otherwise, display as is
            outputElement.textContent = result;
        }
        
        statusElement.textContent = 'Execution completed successfully';
    } catch (error) {
        console.error('Error:', error);
        document.getElementById('status').textContent = 'Error: ' + error.message;
        log_error(error.toString());
    }
}

// Initialize when the page loads
window.addEventListener('load', () => {
    // Initialize WASM
    initWasm();
    
    // Add event listener to the execute button
    document.getElementById('executeBtn').addEventListener('click', executeBinaryAndProxy);
});
