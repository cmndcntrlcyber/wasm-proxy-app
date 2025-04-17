import init, { execute_attack, log_error } from './pkg/wasm_proxy_app.js';

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

// Execute the attack sequence
async function runAttack() {
    const outputElement = document.getElementById('output');
    const statusElement = document.getElementById('status');
    const executeBtn = document.getElementById('executeBtn');
    
    try {
        outputElement.textContent = 'Executing attack sequence...\n';
        executeBtn.disabled = true;
        statusElement.textContent = 'Running...';
        
        // Clear the console
        console.clear();
        
        // Redirect console.log to our output element
        const originalConsoleLog = console.log;
        console.log = function() {
            const args = Array.from(arguments);
            outputElement.textContent += args.join(' ') + '\n';
            originalConsoleLog.apply(console, arguments);
        };
        
        // Execute the attack
        const result = await execute_attack();
        
        // Restore console.log
        console.log = originalConsoleLog;
        
        outputElement.textContent += 'Attack sequence completed.\n';
        outputElement.textContent += 'Result: ' + result + '\n';
        statusElement.textContent = 'Completed successfully.';
    } catch (e) {
        console.error('Error executing attack:', e);
        outputElement.textContent += 'Error: ' + e.message + '\n';
        statusElement.textContent = 'Failed: ' + e.message;
        log_error('JavaScript error: ' + e.message);
    } finally {
        executeBtn.disabled = false;
    }
}

// Set up event listeners
document.addEventListener('DOMContentLoaded', () => {
    const executeBtn = document.getElementById('executeBtn');
    executeBtn.disabled = true; // Disable until WASM is loaded
    executeBtn.addEventListener('click', runAttack);
    
    // Initialize WebAssembly
    initWasm();
});
