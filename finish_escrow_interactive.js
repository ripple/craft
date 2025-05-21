const readline = require('readline');
const { exec } = require('child_process');
const path = require('path');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

// Function to prompt user for input
function prompt(question) {
  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      resolve(answer);
    });
  });
}

async function main() {
  try {
    console.log('Please provide the following details to finish the escrow:');
    
    const account = await prompt('Enter your account address: ');
    const accountSecret = await prompt('Enter your account secret: ');
    const owner = await prompt('Enter the owner address (who created the escrow): ');
    const offerSequence = await prompt('Enter the offer sequence number: ');

    // Close the readline interface
    rl.close();

    // Construct the command to run finish_escrow.js
    const finishEscrowScript = path.join(__dirname, 'finish_escrow.js');
    const command = `node ${finishEscrowScript} ${account} ${accountSecret} ${owner} ${offerSequence}`;

    // Execute the finish_escrow.js script
    exec(command, (error, stdout, stderr) => {
      if (error) {
        console.error(`Error executing finish_escrow.js: ${error.message}`);
        return;
      }
      if (stderr) {
        console.error(`stderr: ${stderr}`);
        return;
      }
      
      // Parse the output to get the transaction hash
      const output = stdout.toString();
      const txHashMatch = output.match(/hash["\s:]+([A-F0-9]+)/i);
      
      if (txHashMatch && txHashMatch[1]) {
        const txHash = txHashMatch[1];
        console.log('\nTransaction submitted successfully!');
        console.log(`View transaction in Explorer: https://livenet.xrpl.org/transactions/${txHash}`);
      } else {
        console.log('\nTransaction output:', output);
      }
    });

  } catch (error) {
    console.error('Error:', error.message);
    rl.close();
  }
}

main(); 