# WASM Devnet Details

## Devnet2 - 2025-05-01 Release

Devnet details:

URL: wasm.devnet.rippletest.net (port 51234 for HTTP, 51233 for WS)

Explorer: https://custom.xrpl.org/wasm.devnet.rippletest.net

Faucet: https://wasmfaucet.devnet.rippletest.net/accounts

Changelog:

There is now a ComputationAllowance field on EscrowFinish that must be included if the escrow has a FinishFunction (i.e. is a Smart Escrow). This specifies the amount of gas that you want to use for your extension. If the extension needs more gas than that, it'll fail. Right now, for testing purposes, the cost is 1 drop per gas (we expect that to be much less on Mainnet) and 1 gas per instruction (this may also change in the future).
(note: there's a known bug where you get tecINTERNAL if you don't specify ComputationAllowance when you need to)

There are some basic DoS prevention limits on size and gas for transactions (100KB and 1 million gas).
A few more host functions (some keylet generators and a function for getting NFTs).
Switching from WasmEdge to WAMR. We'll put out a doc explaining this decision in more detail soon.

**Tooling:**
- Writing Rust WASM extensions: https://github.com/ripple/craft
- Python: xrpl-py v4.2.0b2
- JS: xrpl@4.3.0-smartescrow.2, ripple-binary-codec@2.4.0-smartescrow.1 (you can also use the @smartescrow or @smart-escrow tags)

## Historical WASM Devnets

### Devnet1 - 2025-04-01 Release

We have the first Programmability Devnet ready for use! Here are all the details:

URL: wasm.devnet.rippletest.net (port 51234 for HTTP, 51233 for WS)

Explorer: https://custom.xrpl.org/wasm.devnet.rippletest.net

Faucet: https://wasmfaucet.devnet.rippletest.net/accounts

rippled code: https://github.com/XRPLF/rippled/tree/ripple/smart-escrow

**Tooling:**
- A CLI to help you write Smart Escrows in Rust (plus some examples): https://github.com/ripple/craft
- A sample script: https://gist.github.com/mvadari/728df55a187283e4116bead99b26b49e
- xrpl-py: https://pypi.org/project/xrpl-py/4.2.0b0/
- xrpl.js: https://www.npmjs.com/package/xrpl/v/4.3.0-smartescrow.0 (can be installed with npm install xrpl@smart-escrow)
- ripple-binary-codec: https://www.npmjs.com/package/ripple-binary-codec/v/2.4.0-smartescrow.0 (can be installed with npm install ripple-binary-codec@smart-escrow)

