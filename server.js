const express = require('express');
const https = require('https');
const fs = require('fs');
const path = require('path');
const bodyParser = require('body-parser');
const cors = require('cors');
const { exec } = require('child_process');

// Create Express apps for HTTP and HTTPS servers
const app = express();
const proxyApp = express();

// Middleware
app.use(cors());
app.use(bodyParser.json());
proxyApp.use(cors());
proxyApp.use(bodyParser.json());

// Serve static files from the dist directory
app.use(express.static(path.join(__dirname, 'dist')));

// API endpoint to execute a binary
app.post('/api/execute', (req, res) => {
    const { binPath, args } = req.body;
    
    if (!binPath) {
        return res.status(400).json({ error: 'Binary path is required' });
    }
    
    console.log(`Executing binary: ${binPath} with args: ${args ? args.join(' ') : ''}`);
    
    // Execute the binary as a child process
    exec(`${binPath} ${args ? args.join(' ') : ''}`, (error, stdout, stderr) => {
        if (error) {
            console.error(`Execution error: ${error.message}`);
            return res.status(500).json({ error: error.message, stderr });
        }
        
        if (stderr) {
            console.warn(`stderr: ${stderr}`);
        }
        
        console.log(`stdout: ${stdout}`);
        
        // Return the output
        res.send(stdout);
    });
});

// Proxy endpoint to receive and forward binary output
proxyApp.post('/proxy', (req, res) => {
    const { output } = req.body;
    
    if (!output) {
        return res.status(400).json({ error: 'Output data is required' });
    }
    
    console.log('Received output data for proxying');
    
    // In a real application, you might forward this data to another service
    // For this example, we'll just log it and send it back
    
    // Log the received data (truncated if too large)
    const outputStr = typeof output === 'string' ? output : JSON.stringify(output);
    console.log(`Proxied output (truncated): ${outputStr.substring(0, 200)}${outputStr.length > 200 ? '...' : ''}`);
    
    // Return a success response
    res.json({
        success: true,
        message: 'Output received and proxied successfully',
        timestamp: new Date().toISOString(),
        size: outputStr.length
    });
});

// Default route for the main application
app.get('*', (req, res) => {
    res.sendFile(path.join(__dirname, 'dist', 'index.html'));
});

// Read SSL certificates
// In a production environment, you would use proper certificates
// For development, we'll use self-signed certificates
let httpsOptions;
try {
    httpsOptions = {
        key: fs.readFileSync(path.join(__dirname, 'certs', 'key.pem')),
        cert: fs.readFileSync(path.join(__dirname, 'certs', 'cert.pem'))
    };
} catch (error) {
    console.warn('SSL certificates not found. Please generate them using the instructions in README.md');
    console.warn('Creating a directory for certificates...');
    
    // Create certs directory if it doesn't exist
    if (!fs.existsSync(path.join(__dirname, 'certs'))) {
        fs.mkdirSync(path.join(__dirname, 'certs'));
    }
    
    // Create placeholder files with instructions
    fs.writeFileSync(
        path.join(__dirname, 'certs', 'README.txt'),
        'Generate self-signed certificates with the following commands:\n\n' +
        'openssl genrsa -out key.pem 2048\n' +
        'openssl req -new -key key.pem -out csr.pem\n' +
        'openssl x509 -req -days 365 -in csr.pem -signkey key.pem -out cert.pem\n' +
        'rm csr.pem\n'
    );
    
    // Use default options for now (this will cause an error when starting HTTPS server)
    httpsOptions = {
        key: 'MISSING',
        cert: 'MISSING'
    };
}

// Start HTTP server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`HTTP Server running on http://localhost:${PORT}`);
});

// Start HTTPS server for the proxy
const HTTPS_PORT = process.env.HTTPS_PORT || 3001;
try {
    if (httpsOptions.key !== 'MISSING' && httpsOptions.cert !== 'MISSING') {
        https.createServer(httpsOptions, proxyApp).listen(HTTPS_PORT, () => {
            console.log(`HTTPS Proxy Server running on https://localhost:${HTTPS_PORT}`);
        });
    } else {
        console.error('HTTPS server could not be started due to missing certificates');
        console.log('Please generate SSL certificates and restart the server');
    }
} catch (error) {
    console.error('Failed to start HTTPS server:', error.message);
}
