const { Client } = require('xrpl');

const client = new Client('ws://127.0.0.1:6006');

async function acceptLedger() {
  try {
    await client.connect();
    console.log('Connected to rippled');
    
    const response = await client.request({
      command: 'ledger_accept'
    });
    
    console.log('Ledger accepted:', JSON.stringify(response, null, 2));
  } catch (error) {
    console.error('Error:', error);
  } finally {
    // await client.disconnect();
  }
}

acceptLedger();

setInterval(() => 
  {
    if (client.isConnected())
      client.request({command: 'ledger_accept'})
  }, 2000)