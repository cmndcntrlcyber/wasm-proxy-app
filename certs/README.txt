Generate self-signed certificates with the following commands:

openssl genrsa -out key.pem 2048
openssl req -new -key key.pem -out csr.pem
openssl x509 -req -days 365 -in csr.pem -signkey key.pem -out cert.pem
rm csr.pem

When prompted for certificate information, you can use the following values for a local development environment:
- Country Name: US (or your country)
- State or Province: Your state
- Locality Name: Your city
- Organization Name: Development
- Organizational Unit: Local Development
- Common Name: localhost
- Email Address: your.email@example.com

Note: These certificates are for development purposes only and should not be used in production.
