const xrpl = require("xrpl")
const fs = require('fs')
const path = require('path')

const sleep = ms => new Promise(r => setTimeout(r, ms))

if (process.argv.length < 3) {
  console.error(
    'Usage: ' +
      process.argv[0] +
      ' ' +
      process.argv[1] +
      ' (path/to/file.wasm OR project_name)' +
      '[Account1 Account1Seed [Account2 Account2Seed]]',
  )
  process.exit(1)
}

const client = new xrpl.Client("ws://127.0.0.1:6006")

function getFinishFunctionFromFile(filePath) {
  if (!filePath) {
    console.error("Please provide a file path as a CLI argument.")
    process.exit(1)
  }

  let absolutePath = ""
  if (filePath.endsWith('.wasm')) {
    absolutePath = path.resolve(filePath)
  } else {
    absolutePath = path.resolve(__dirname, `../../projects/${filePath}/target/wasm32-unknown-unknown/release/${filePath}.wasm`)
  }
  try {
    const data = fs.readFileSync(absolutePath)
    return data.toString('hex')
  } catch (err) {
    console.error(`Error reading file at ${absolutePath}:`, err.message)
    process.exit(1)
  }
}

async function submit(tx, wallet, debug = true) {
  const txResult = await client.submitAndWait(tx, {autofill: true, wallet})
  console.log("SUBMITTED " + tx.TransactionType)

  if (debug)
    console.log(txResult.result ?? txResult)
  else
    console.log("Result code: " + txResult.result?.meta?.TransactionResult)
  return txResult
}

async function fundWallet(wallet = undefined) {
  const master = xrpl.Wallet.fromSeed("snoPBrXtMeMyMHUVTgbuqAfg1SUTb", { algorithm: xrpl.ECDSA.secp256k1 })

  const walletToFund = wallet || xrpl.Wallet.generate()
  await submit({
    TransactionType: 'Payment',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: xrpl.xrpToDrops(10000),
    Destination: walletToFund.address,
  }, master)
  return { walletToFund }
}

async function deploy() {
  await client.connect()
  console.log("connected")
  await client.request({command: 'ledger_accept'})

  const interval = setInterval(() => {if (client.isConnected()) client.request({command: 'ledger_accept'})},1000)

  let wallet, wallet2
  if (process.argv.length > 3) {
    const account = process.argv[3]
    const accountSecret = process.argv[4]
    console.log(account, accountSecret)
    wallet = xrpl.Wallet.fromSeed(accountSecret, {masterAddress: account})

    if (process.argv.length > 5) {
      const account2 = process.argv[5]
      const accountSecret2 = process.argv[6]
      wallet2 = xrpl.Wallet.fromSeed(accountSecret2, {masterAddress: account2})
    } else {
      wallet2 = xrpl.Wallet.generate()
    }
  } else {
    wallet = xrpl.Wallet.generate()
    wallet2 = xrpl.Wallet.generate()
  }

  const finish = getFinishFunctionFromFile(process.argv[2])

  await fundWallet(wallet)
  await fundWallet(wallet2)

  console.log(`\nFunded accounts:`)
  console.log(`Account 1 (Origin) - Address: ${wallet.address}`)
  console.log(`Account 1 (Origin) - Secret: ${wallet.seed}`)
  console.log(`Account 2 (Destination) - Address: ${wallet2.address}`)
  console.log(`Account 2 (Destination) - Secret: ${wallet2.seed}\n`)

  const close_time = (
    await client.request({
      command: 'ledger',
      ledger_index: 'validated',
    })
  ).result.ledger.close_time

  const response1 = await submit({
    TransactionType: 'EscrowCreate',
    Account: wallet.address,
    Amount: "100000",
    Destination: wallet2.address,
    CancelAfter: close_time + 2000,
    FinishAfter: close_time + 5,
    FinishFunction: finish,
    Data: xrpl.xrpToDrops(70),
  }, wallet)

  if (response1.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1)
  const sequence = response1.result.tx_json.Sequence

  await client.disconnect()
  clearInterval(interval)
}



deploy()
