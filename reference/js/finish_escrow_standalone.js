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

const url = "ws://127.0.0.1:6006"
const client = new xrpl.Client(url)

const [, , account, accountSecret, owner, offerSequence] = process.argv

async function submit(tx, wallet, debug = true) {
  const result = await client.submitAndWait(tx, {autofill: true, wallet})
  console.log("SUBMITTED " + tx.TransactionType)
  if (debug)
    console.log(result.result ?? result)
  else
    console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function finishEscrow() {
  let interval
  try {
    await client.connect()
    console.log(`Connected to ${url}`)
    await client.request({command: 'ledger_accept'})

    interval = setInterval(() => {if (client.isConnected()) client.request({command: 'ledger_accept'})},1000)

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
