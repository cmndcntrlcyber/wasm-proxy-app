import init, { execute_attack, log_error, get_execution_output, is_execution_in_progress } from './pkg/wasm_proxy_app.js';

// Initialize the WebAssembly module
async function initWasm() {
    try {
        await init();
        document.getElementById('status').textContent = 'WebAssembly module loaded successfully.';
        document.getElementById('executeBtn').disabled = false;
    } catch (e) {
        console.error('Failed to initialize WebAssembly module:', e);
        document.getElementById('status').textContent = 'Failed to load WebAssembly module: ' + e.message;
        document.getElementById('executeBtn').disabled = true;
    }
}

// Execute the rust-run application
async function executeRustRun() {
    const outputElement = document.getElementById('output');
    const statusElement = document.getElementById('status');
    const executeBtn = document.getElementById('executeBtn');
    
    // Check if execution is already in progress
    if (is_execution_in_progress()) {
        alert('Execution already in progress. Please wait for it to complete.');
        return;
    }
    
    try {
        // Update UI
        outputElement.textContent = 'Executing rust-run application...\n';
        executeBtn.disabled = true;
        statusElement.textContent = 'Running...';
        
        // Execute the rust-run binary
        const result = await execute_attack();
        
        // Display execution result
        outputElement.textContent += `Execution complete.\n`;
        outputElement.textContent += `Result: ${result}\n`;
        
        // Get full output
        const fullOutput = get_execution_output();
        if (fullOutput && fullOutput.trim().length > 0) {
            outputElement.textContent += '\n--- Detailed Output ---\n' + fullOutput + '\n';
        }
        
        // Update status
        statusElement.textContent = 'Completed successfully.';
    } catch (e) {
        console.error('Error executing rust-run:', e);
        outputElement.textContent += 'Error: ' + e.message + '\n';
        statusElement.textContent = 'Failed: ' + e.message;
        log_error('JavaScript error: ' + e.message);
    } finally {
        executeBtn.disabled = false;
    }
}

// Listen for real-time output from rust-run
function setupOutputListener() {
    window.addEventListener('rust-run-output', (event) => {
        const outputElement = document.getElementById('output');
        
        // Append the new output line
        if (event.detail && event.detail.output) {
            outputElement.textContent += event.detail.output + '\n';
            
            // Auto-scroll to bottom
            outputElement.scrollTop = outputElement.scrollHeight;
        }
    });
}

// Set up event listeners
document.addEventListener('DOMContentLoaded', () => {
    const executeBtn = document.getElementById('executeBtn');
    executeBtn.disabled = true; // Disable until WASM is loaded
    executeBtn.addEventListener('click', executeRustRun);
    
    // Set up output listener
    setupOutputListener();
    
    // Initialize WebAssembly
    initWasm();
});
