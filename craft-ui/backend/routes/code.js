const express = require('express');
const router = express.Router();
const fs = require('fs').promises;
const path = require('path');

// Base path to KYC project
const KYC_PROJECT_PATH = path.join(__dirname, '../../../projects/kyc/src');

// Get KYC project code
router.get('/kyc', async (req, res) => {
  try {
    const mainFile = path.join(KYC_PROJECT_PATH, 'lib.rs');
    const code = await fs.readFile(mainFile, 'utf8');
    res.json({ code });
  } catch (error) {
    console.error('Error reading KYC code:', error);
    res.status(500).json({ error: 'Failed to read KYC project code' });
  }
});

// Update KYC project code
router.put('/kyc', async (req, res) => {
  try {
    const { code } = req.body;
    if (!code) {
      return res.status(400).json({ error: 'Code is required' });
    }
    
    const mainFile = path.join(KYC_PROJECT_PATH, 'lib.rs');
    await fs.writeFile(mainFile, code, 'utf8');
    res.json({ message: 'Code updated successfully' });
  } catch (error) {
    console.error('Error updating KYC code:', error);
    res.status(500).json({ error: 'Failed to update KYC project code' });
  }
});

module.exports = router;