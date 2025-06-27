const express = require('express');
const router = express.Router();
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs').promises;

// Submit EscrowFinish transaction
router.post('/escrow/finish', (req, res) => {
  const { account, secret, owner, sequence, server } = req.body;
  
  if (!account || !secret || !owner || !sequence || !server) {
    return res.status(400).json({ 
      error: 'Missing required fields: account, secret, owner, sequence, server' 
    });
  }
  
  // Validate XRPL address formats
  if (!/^r[a-zA-Z0-9]{24,34}$/.test(account)) {
    return res.status(400).json({ error: 'Invalid account address format' });
  }
  if (!/^r[a-zA-Z0-9]{24,34}$/.test(owner)) {
    return res.status(400).json({ error: 'Invalid owner address format' });
  }
  
  // Validate secret format
  if (!/^s[a-zA-Z0-9]{20,}$/.test(secret)) {
    return res.status(400).json({ error: 'Invalid secret format' });
  }
  
  // Validate sequence is a number
  if (isNaN(sequence) || sequence < 0) {
    return res.status(400).json({ error: 'Invalid sequence number' });
  }
  
  console.log('Finishing escrow:', { account, owner, sequence });
  
  const finishScript = path.join(__dirname, '../../../reference/js/escrow_finish.js');
  
  const args = [
    finishScript,
    server,
    account,
    secret,
    owner,
    sequence.toString()
  ];
  
  console.log('Executing command: node', args.join(' '));
  console.log('Arguments passed:', args);
  
  const finishProcess = spawn('node', args, {
    cwd: path.join(__dirname, '../../../reference/js'),
    env: { ...process.env }
  });
  
  let output = '';
  let error = '';
  let result = null;
  
  finishProcess.stdout.on('data', (data) => {
    const text = data.toString();
    output += text;
    console.log('Finish output:', text);
    
    // Check for success or failure
    if (text.includes('tesSUCCESS')) {
      result = 'SUCCESS';
    } else if (text.includes('Failed to finish escrow')) {
      result = 'FAILED';
    }
  });
  
  finishProcess.stderr.on('data', (data) => {
    error += data.toString();
    console.error('Finish error:', data.toString());
  });
  
  finishProcess.on('close', (code) => {
    res.json({
      success: code === 0,
      result,
      output,
      error
    });
  });
});

// Create KYC credential
router.post('/credential/create', async (req, res) => {
  const { account, secret, subject, server, credentialType = 'KYC' } = req.body;
  
  if (!account || !secret || !subject || !server) {
    return res.status(400).json({ 
      error: 'Missing required fields: account, secret, subject, server' 
    });
  }
  
  // Validate XRPL address format
  if (!/^r[a-zA-Z0-9]{24,34}$/.test(account)) {
    return res.status(400).json({ error: 'Invalid XRPL address format' });
  }
  
  // Basic validation for secret format
  if (!/^s[a-zA-Z0-9]{20,}$/.test(secret)) {
    return res.status(400).json({ error: 'Invalid secret format' });
  }
  
  console.log('Creating credential:', { account, subject });
  
  const credScript = path.join(__dirname, '../../../reference/js/create_credential.js');
  
  const credArgs = [
    credScript,
    server,
    account,
    secret,
    subject
  ];
  
  console.log('Executing command: node', credArgs.join(' '));
  console.log('Arguments passed:', credArgs);
  
  const credProcess = spawn('node', credArgs, {
    cwd: path.join(__dirname, '../../../reference/js'),
    env: { ...process.env }
  });
  
  let output = '';
  let error = '';
  
  credProcess.stdout.on('data', (data) => {
    output += data.toString();
    console.log('Credential output:', data.toString());
  });
  
  credProcess.stderr.on('data', (data) => {
    error += data.toString();
    console.error('Credential error:', data.toString());
  });
  
  credProcess.on('close', async (code) => {
    res.json({
      success: code === 0,
      output,
      error
    });
  });
});

module.exports = router;