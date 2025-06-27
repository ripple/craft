const express = require('express');
const router = express.Router();
const { spawn } = require('child_process');
const path = require('path');

// Deploy to WASM Devnet
router.post('/', (req, res) => {
  const { server = 'wss://wasm.devnet.rippletest.net:51233', wasmPath } = req.body;
  
  if (!wasmPath) {
    return res.status(400).json({ error: 'WASM path is required' });
  }
  
  console.log('Deploying to:', server);
  console.log('WASM path:', wasmPath);
  
  // Use the deploy_sample.js script with server parameter
  const deployScript = path.join(__dirname, '../../../reference/js/deploy_sample.js');
  
  const deployProcess = spawn('node', [deployScript, wasmPath, server], {
    cwd: path.join(__dirname, '../../../reference/js'),
    env: { ...process.env }
  });
  
  let output = '';
  let error = '';
  let deploymentInfo = {};
  
  deployProcess.stdout.on('data', (data) => {
    const text = data.toString();
    output += text;
    console.log('Deploy output:', text);
  });
  
  deployProcess.stderr.on('data', (data) => {
    error += data.toString();
    console.error('Deploy error:', data.toString());
  });
  
  deployProcess.on('close', (code) => {
    console.log('Deploy process closed with code:', code);
    
    // Parse deployment information from complete output
    const account1Match = output.match(/Account 1(?:\s*\(Origin\))?.*Address:\s*(\w+)/);
    const secret1Match = output.match(/Account 1(?:\s*\(Origin\))?.*Secret:\s*(\w+)/);
    const account2Match = output.match(/Account 2(?:\s*\(Destination\))?.*Address:\s*(\w+)/);
    const secret2Match = output.match(/Account 2(?:\s*\(Destination\))?.*Secret:\s*(\w+)/);
    
    if (account1Match) deploymentInfo.account1 = account1Match[1];
    if (secret1Match) deploymentInfo.secret1 = secret1Match[1];
    if (account2Match) deploymentInfo.account2 = account2Match[1];
    if (secret2Match) deploymentInfo.secret2 = secret2Match[1];
    
    // Look for the Sequence from the tx_json of EscrowCreate - get the LAST occurrence
    const sequenceMatches = output.match(/Sequence:\s*(\d+)/g);
    if (sequenceMatches && sequenceMatches.length > 0) {
      // Get the last match
      const lastMatch = sequenceMatches[sequenceMatches.length - 1];
      const sequenceNumber = lastMatch.match(/\d+/)[0];
      deploymentInfo.escrowSequence = sequenceNumber;
      console.log('Found escrow sequence (last occurrence):', sequenceNumber);
    } else {
      console.log('No sequence match found in complete output');
    }
    
    console.log('Final deployment info:', deploymentInfo);
    
    if (code === 0 || (deploymentInfo.account1 && deploymentInfo.account2)) {
      // Save deployment info
      deploymentInfo.timestamp = new Date().toISOString();
      deploymentInfo.server = server;
      req.app.setDeploymentInfo(deploymentInfo);
      
      res.json({
        success: true,
        deployment: deploymentInfo,
        output
      });
    } else {
      res.status(500).json({
        success: false,
        error: error || output,
        code
      });
    }
  });
});

module.exports = router;