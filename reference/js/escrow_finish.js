const xrpl = require("xrpl")

console.log('escrow_finish.js received arguments:', process.argv)
console.log('Argument count:', process.argv.length)

if (process.argv.length !== 7) {
  console.error('Usage: node escrow_finish.js <server> <account> <secret> <owner> <sequence>')
  console.error('Example: node escrow_finish.js wss://wasm.devnet.rippletest.net:51233 rAccount... sSecret... rOwner... 123')
  process.exit(1)
}

const [,, server, account, secret, owner, sequence] = process.argv
console.log('Parsed arguments:', { server, account, secret: secret.substring(0, 5) + '...', owner, sequence })

async function finishEscrow() {
  const client = new xrpl.Client(server)
  
  try {
    console.log(`Connecting to ${server}...`)
    await client.connect()
    console.log('Connected successfully')
    
    const wallet = xrpl.Wallet.fromSeed(secret)
    
    // Verify the account matches the wallet
    if (wallet.address !== account) {
      throw new Error(`Account mismatch: expected ${account}, got ${wallet.address}`)
    }
    
    const tx = {
      TransactionType: 'EscrowFinish',
      Account: account,
      Owner: owner,
      OfferSequence: parseInt(sequence),
      ComputationAllowance: 1000000
    }
    
    console.log('Submitting EscrowFinish transaction...')
    console.log(JSON.stringify(tx, null, 2))
    
    const result = await client.submitAndWait(tx, { wallet })
    
    console.log('\nTransaction Result:', result.result.meta.TransactionResult)
    
    if (result.result.meta.TransactionResult === 'tesSUCCESS') {
      console.log('✅ EscrowFinish transaction succeeded!')
    } else {
      console.log('❌ EscrowFinish transaction failed')
    }
    
    console.log('\nFull result:')
    console.log(JSON.stringify(result, null, 2))
    
  } catch (error) {
    console.error('Error:', error.message)
    process.exit(1)
  } finally {
    await client.disconnect()
    console.log('\nDisconnected from server')
  }
}

finishEscrow()