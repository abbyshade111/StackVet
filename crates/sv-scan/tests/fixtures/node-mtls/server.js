// A server that asks every client for a certificate (mutual TLS).
const https = require("https");
const fs = require("fs");

const server = https.createServer(
  {
    key: fs.readFileSync("server-key.pem"),
    cert: fs.readFileSync("server-cert.pem"),
    ca: fs.readFileSync("clients-ca.pem"),
    requestCert: true,
    rejectUnauthorized: true,
  },
  (req, res) => {
    const who = req.socket.getPeerCertificate().subject;
    res.end(`hello ${who && who.CN}`);
  }
);
server.listen(8443);
