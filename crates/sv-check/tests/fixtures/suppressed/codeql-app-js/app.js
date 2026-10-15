const express = require('express');
const cp = require('child_process');
const app = express();

app.get('/control', (req, res) => {
  cp.exec('ls ' + req.query.a);
  res.send('ok');
});

app.get('/lgtm-same-line', (req, res) => {
  cp.exec('ls ' + req.query.b); // lgtm[js/command-line-injection]
  res.send('ok');
});

app.get('/codeql-line-before', (req, res) => {
  // codeql[js/command-line-injection]
  cp.exec('ls ' + req.query.c);
  res.send('ok');
});

app.get('/codeql-same-line', (req, res) => {
  cp.exec('ls ' + req.query.d); // codeql[js/command-line-injection]
  res.send('ok');
});

app.listen(3000);
