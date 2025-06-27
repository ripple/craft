const xrpl = require("xrpl")

if (process.argv.length != 6) {
  console.error(
    'Usage: ' +
      process.argv[0] +
      ' ' +
      process.argv[1] +
      ' <Account> <AccountSecret> <Owner> <OfferSequence>',
  )
  process.exit(1)
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// const url = "ws://127.0.0.1:6006"
const url = "wss://wasm.devnet.rippletest.net:51233"
const client = new xrpl.Client(url)

const [, , account, accountSecret, owner, offerSequence] = process.argv

// Write out the FULL tx response to a file
// Create/open new file for all txs to go in at once,
// for all tx types:
const txResponseFile = require('path').resolve(__dirname, '../../tx_responses.json')

async function submit(tx, wallet, debug = true) {
  const autofilledTx = await client.autofill(tx)

  // Write autofilledTx to the file
  require('fs').appendFileSync(txResponseFile, JSON.stringify(autofilledTx, null, 2) + ',\n', 'utf8', (err) => {
    if (err) {
      console.error("Error writing to file:", err.message)
    }
  })

  const result = await client.submitAndWait(autofilledTx, {autofill: false, wallet})
  console.log("SUBMITTED " + autofilledTx.TransactionType)
  if (debug)
    console.log(result.result ?? result)
  else
    console.log("Result code: " + result.result?.meta?.TransactionResult)

  // Append the full transaction response to the file
  require('fs').appendFileSync(txResponseFile, JSON.stringify(result, null, 2) + ',\n', 'utf8', (err) => {
    if (err) {
      console.error("Error writing to file:", err.message)
    }
  })

  return result
}

async function finishEscrow() {
  let interval
  try {
    await client.connect()
    console.log(`Connected to ${url}`)


    // await client.request({command: 'ledger_accept'})
    // interval = setInterval(() => {if (client.isConnected()) client.request({command: 'ledger_accept'})},1000)

    const wallet = xrpl.Wallet.fromSeed(accountSecret)
    
    // Verify the account matches the wallet
    if (wallet.address !== account) {
      console.error("Error: Provided account doesn't match the wallet derived from the secret")
      process.exit(1)
    }

    console.log("\nTransaction Details:")
    console.log(`Account (Finishing Escrow): ${account}`)
    console.log(`Owner (Created Escrow): ${owner}`)
    console.log(`Offer Sequence: ${offerSequence}\n`)

    await sleep(2000)

    const txFail = {
      TransactionType: 'EscrowFinish',
      Account: account,
      Owner: owner,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    console.log("Submitting EscrowFinish transaction... (this should fail)")
    
    const responseFail = await submit(txFail, wallet)
    
    if (responseFail.result.meta.TransactionResult === "tesSUCCESS") {
      console.log("\nEscrow finished successfully!")
    } else {
      console.error("\nFailed to finish escrow:", responseFail.result.meta.TransactionResult)
    }

    await sleep(2000)

    const credTx = {
      TransactionType: 'CredentialCreate',
      Account: account,
      Subject: account,
      CredentialType: xrpl.convertStringToHex('termsandconditions'),
      URI: xrpl.convertStringToHex("https://example.com/terms"),
    }

    console.log("Submitting CredentialCreate transaction...")
    const credResponse = await submit(credTx, wallet)
    
    if (credResponse.result.meta.TransactionResult === "tesSUCCESS") {
      console.log("Credential created successfully!")
    } else {
      console.error("\nFailed to create credential:", credResponse.result.meta.TransactionResult)
    }

    await sleep(2000)

    const tx = {
      TransactionType: 'EscrowFinish',
      Account: account,
      Owner: owner,
      OfferSequence: parseInt(offerSequence),
      ComputationAllowance: 1000000,
    }

    console.log("Submitting EscrowFinish transaction...")
    const response = await submit(tx, wallet)
    
    if (response.result.meta.TransactionResult === "tesSUCCESS") {
      console.log("\nEscrow finished successfully!")
    } else {
      console.error("\nFailed to finish escrow:", response.result.meta.TransactionResult)
    }

  } catch (error) {
    console.error("Error:", error.message)
  } finally {
    await client.disconnect()
    clearInterval(interval)
    console.log("Disconnected")
  }
}

finishEscrow() 
