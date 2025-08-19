// Faucet helper: creates and funds a Devnet account, then prints export lines.
// Usage:
//   node faucet.js
// Output:
//   Prints lines you can copy/paste into bash/zsh:
//     export NOTARY_ADDRESS=...
//     export NOTARY_SEED=...
// Notes:
//   - Uses WASM Devnet endpoint wss://wasm.devnet.rippletest.net:51233
//   - This script does not modify your shell; you must copy/paste the exports.

const xrpl = require("xrpl")

async function main() {
  const url = "wss://wasm.devnet.rippletest.net:51233"
  const client = new xrpl.Client(url)
  try {
    await client.connect()
    const { wallet } = await client.fundWallet()
    console.log("# Copy and paste these lines into your shell to export the variables:")
    console.log(`export NOTARY_ADDRESS=${wallet.address}`)
    console.log(`export NOTARY_SEED=${wallet.seed}`)
  } catch (e) {
    console.error("Faucet error:", e.message)
    process.exit(1)
  } finally {
    await client.disconnect()
  }
}

main()
