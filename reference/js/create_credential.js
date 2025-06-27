const xrpl = require("xrpl")

console.log('create_credential.js received arguments:', process.argv)
console.log('Argument count:', process.argv.length)

if (process.argv.length !== 6) {
  console.error('Usage: node create_credential.js <server> <account> <secret> <subject>')
  console.error('Example: node create_credential.js wss://wasm.devnet.rippletest.net:51233 rAccount... sSecret... rSubject...')
  process.exit(1)
}

const [,, server, account, secret, subject] = process.argv
console.log('Parsed arguments:', { server, account, secret: secret.substring(0, 5) + '...', subject })

async function createCredential() {
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
    
    const credTx = {
      TransactionType: 'CredentialCreate',
      Account: account,
      Subject: subject,
      CredentialType: xrpl.convertStringToHex('termsandconditions'),
      URI: xrpl.convertStringToHex("https://example.com/terms"),
    }
    
    console.log('Creating credential...')
    console.log('\nSubmitting transaction:')
    console.log(JSON.stringify(credTx, null, 2))
    
    const result = await client.submitAndWait(credTx, { wallet })
    
    console.log('\nTransaction Result:', result.result.meta.TransactionResult)
    
    if (result.result.meta.TransactionResult === 'tesSUCCESS') {
      console.log('✅ Credential created successfully!')
    } else {
      console.log('❌ Failed to create credential:', result.result.meta.TransactionResult)
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

createCredential()