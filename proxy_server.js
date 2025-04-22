
const http = require('http');
const { exec } = require('child_process');
const url = require('url');

const server = http.createServer((req, res) => {
    const parsedUrl = url.parse(req.url, true);
    
    if (parsedUrl.pathname === '/execute-binary') {
        const binaryPath = parsedUrl.query.path;
        
        console.log(`Executing binary: ${binaryPath}`);
        
        exec(binaryPath, (error, stdout, stderr) => {
            res.setHeader('Content-Type', 'text/plain');
            
            if (error) {
                console.error(`Execution error: ${error.message}`);
                res.statusCode = 500;
                res.end(`Error executing binary: ${error.message}\n${stderr}`);
                return;
            }
            
            if (stderr) {
                console.warn(`Command stderr: ${stderr}`);
            }
            
            console.log(`Command output: ${stdout}`);
            res.statusCode = 200;
            res.end(stdout);
        });
    } else {
        // Serve static files or other endpoints
        res.statusCode = 404;
        res.end('Not Found');
    }
});

const PORT = 3000;
server.listen(PORT, () => {
    console.log(`Proxy server running at http://localhost:${PORT}/`);
});
