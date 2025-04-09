const xrpl = require("xrpl")
const fs = require('fs')
const path = require('path')

const sleep = ms => new Promise(r => setTimeout(r, ms))

if (process.argv.length != 3) {
  console.error(
    'Usage: ' +
      process.argv[0] +
      ' ' +
      process.argv[1] +
      ' path/to/file.wasm',
  )
  process.exit(1)
}

const client = new xrpl.Client("wss://wasm.devnet.rippletest.net:51233")

function getFinishFunctionFromFile(filePath) {
  if (!filePath) {
    console.error("Please provide a file path as a CLI argument.")
    process.exit(1)
  }

  const absolutePath = path.resolve(filePath)
  try {
    const data = fs.readFileSync(absolutePath, 'utf8').trim()
    return data.replace("\n","").replace(" ", "")
  } catch (err) {
    console.error(`Error reading file at ${absolutePath}:`, err.message)
    process.exit(1)
  }
}

async function submit(tx, seed, debug = true) {
  tx.Fee = "10000"
  const wallet = xrpl.Wallet.fromSeed(seed)
  const result = await client.submitAndWait(tx, {autofill: true, wallet})
  console.log("SUBMITTED " + tx.TransactionType)
  if (debug)
    console.log(result.result ?? result)
  else
    console.log("Result code: " + result.result?.meta?.TransactionResult)
  return result
}

async function deploy() {
  await client.connect()
  console.log("connected")
  const {wallet} = await client.fundWallet(null)
  const {wallet: wallet2 } = await client.fundWallet(null)
  console.log(`\nFunded accounts:`)
  console.log(`Account 1 - Address: ${wallet.address}`)
  console.log(`Account 1 - Secret: ${wallet.seed}`)
  console.log(`Account 2 - Address: ${wallet2.address}`)
  console.log(`Account 2 - Secret: ${wallet2.seed}\n`)

  const close_time = (
    await client.request({
      command: 'ledger',
      ledger_index: 'validated',
    })
  ).result.ledger.close_time

  const filePath = process.argv[2]
  const finish = getFinishFunctionFromFile(filePath)

  const response1 = await submit({
    TransactionType: 'EscrowCreate',
    Account: wallet.address,
    Amount: "100000",
    Destination: wallet2.address,
    CancelAfter: close_time + 200,
    FinishAfter: close_time + 5,
    FinishFunction: finish,
    Data: xrpl.xrpToDrops(70),
  }, wallet.seed, true)
  // console.log(JSON.stringify(response1.result, null, 4))
  if (response1.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1)
  const sequence = response1.result.tx_json.Sequence

  await client.disconnect()
}

deploy()
