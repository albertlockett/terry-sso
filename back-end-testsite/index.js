const express = require('express');
const cors = require('cors');
const jwt = require('jsonwebtoken');

const app = express();
app.use(cors());

const publicKey = `-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA8f/E4EJRrr8jgn1qRJLb
rJjpFKuc0haRb9HFpc5fJeBq2xfJpu5gnuKsFInt5vNJL1Zho17VCg0LYC3XzCHU
ywlQfjMkzA1BrSrDD58yx4eWwSlIyR6r9dikJYJm6Ho5c4+wCKga+2YFrgBZp9BK
UHHCWbi8otE+rdPL/8K+mxnqt/R4Pm2QvFBRrJ433m5nZx024Fr9DMP59Sl0QjOC
fxzWr33Juyr5nURUxl0IcSg/BjPqQN4j8qTeADLB/BEQVvfJuBc1dy0IpOIhnrkM
9Gk0GfCePKYvFCHn90yROZvriukcFMFaNQtbNkEHr2Uh4VVDYI/8FgHGSWqPr49J
GwIDAQAB
-----END PUBLIC KEY-----`;

app.get('/v1/data', function(req, res) {
  const authHeader = req.get('Authorization');
  const token = authHeader.split(' ')[1];

  try {
    jwt.verify(token, publicKey);
    res.status(200);
    res.json({ data: 'OK' })
  } catch (e) {
    res.status(403);
    res.json({ error: 'unauthorized' })
  }
});

app.listen(4001, function() {
  console.log('listening on port 4001');
});

