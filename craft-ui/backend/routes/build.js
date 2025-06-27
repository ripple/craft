const express = require('express');
const router = express.Router();
const { spawn } = require('child_process');
const path = require('path');

// Build WASM module
router.post('/', (req, res) => {
  const { buildMode = 'release', optimization = 'z' } = req.body;
  
  console.log('Building KYC project with:', { buildMode, optimization });
  
  // Path to craft executable - try multiple locations
  const possiblePaths = [
    path.join(__dirname, '../../../target/release/craft'),
    path.join(__dirname, '../../../target/debug/craft'),
    'craft' // In PATH
  ];
  
  const { execSync } = require('child_process');
  let craftPath = 'craft';
  
  for (const p of possiblePaths) {
    try {
      execSync(`${p} --version`, { stdio: 'ignore' });
      craftPath = p;
      break;
    } catch (e) {
      // Continue to next path
    }
  }
  
  // Build command arguments based on mode
  const args = ['-y', 'build', 'kyc'];
  
  // Always specify mode explicitly
  args.push('--mode', buildMode);
  
  if (optimization) {
    args.push('--opt', optimization);
  }
  
  const buildProcess = spawn(craftPath, args, {
    cwd: path.join(__dirname, '../../..'),
    env: { ...process.env },
    stdio: ['pipe', 'pipe', 'pipe']
  });
  
  let output = '';
  let error = '';
  
  buildProcess.stdout.on('data', (data) => {
    output += data.toString();
    console.log('Build output:', data.toString());
  });
  
  buildProcess.stderr.on('data', (data) => {
    error += data.toString();
    console.error('Build error:', data.toString());
  });
  
  buildProcess.on('close', (code) => {
    if (code === 0) {
      // Extract WASM path and fingerprint from output
      const wasmPathMatch = output.match(/WASM file location:\s*(.+\.wasm)/);
      const fingerprintMatch = output.match(/WASM Fingerprint:\s*(\w+)/);
      
      res.json({
        success: true,
        wasmPath: wasmPathMatch ? wasmPathMatch[1].trim() : null,
        fingerprint: fingerprintMatch ? fingerprintMatch[1] : null,
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