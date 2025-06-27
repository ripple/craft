const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const path = require('path');
const fs = require('fs').promises;

const app = express();
const PORT = 3001;

// Middleware
app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// In-memory storage for deployment info
let latestDeployment = null;

// Import routes
const codeRoutes = require('./routes/code');
const buildRoutes = require('./routes/build');
const deployRoutes = require('./routes/deploy');
const transactionRoutes = require('./routes/transactions');

// Use routes
app.use('/api/code', codeRoutes);
app.use('/api/build', buildRoutes);
app.use('/api/deploy', deployRoutes);
app.use('/api', transactionRoutes);

// Get deployment info
app.get('/api/deployment-info', (req, res) => {
  if (latestDeployment) {
    res.json(latestDeployment);
  } else {
    res.status(404).json({ error: 'No deployment info available' });
  }
});

// Set deployment info (used by other routes)
app.setDeploymentInfo = (info) => {
  latestDeployment = info;
  // Also save to file
  const filePath = path.join(__dirname, '../../tx_responses.json');
  fs.appendFile(filePath, JSON.stringify(info, null, 2) + ',\n')
    .catch(err => console.error('Error saving deployment info:', err));
};

// Start server
app.listen(PORT, () => {
  console.log(`Craft UI backend running on http://localhost:${PORT}`);
});