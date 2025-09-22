# WASM Devnet Details

## Devnet6 - 2025-09-12 Release

The WASM Devnet (Smart Escrows) has been updated to its sixth monthly release.

The code is essentially feature-complete and moving onto the review and testing phase.

The `rippled` code can be found in this branch: https://github.com/XRPLF/rippled/tree/ripple/smart-escrow

Commit: 4d4a1cfe8238be9830c7712e5879cbefd5f10668

Devnet details:

- URL: wasm.devnet.rippletest.net (port 51234 for HTTP, 51233 for WS)
- Explorer: https://custom.xrpl.org/wasm.devnet.rippletest.net
- Faucet: https://wasmfaucet.devnet.rippletest.net/accounts

`rippled` Changelog:

- Update the fees and reserves
  - `EscrowCreate` for a Smart Escrow now costs 100 drops + 5 drops per byte in the `FinishFunction`, and a Smart Escrow is charged 1 additional object reserve per 500 bytes (beyond the first 500 bytes, which are included in the first object reserve)
- Adjust the function signatures of `get_ledger_sqn` and `get_parent_ledger_time` to return the value instead of a buffer
  - This is already reflected in XLS-102
- Add the last set of host functions - mostly some missing keylets, like `MPTokenIssuance`
- Various bugfixes and cleaning up code

Craft Changelog:

- Support the new `rippled` features mentioned above
- Lots of additional documentation - new docs page: https://ripple.github.io/craft/
- Significant amounts of code cleanup and reorganization
  - We have been revamping the repo structure to improve the dev experience
  - Examples are now in `projects/examples`
- Various bugfixes and cleaning up code

Tooling:
- Writing Rust WASM extensions: https://github.com/ripple/craft
- Python: `xrpl-py v4.4.0b1`
- JS: `xrpl@4.5.0-smartescrow.2`, `ripple-binary-codec@2.6.0-smartescrow.2` (you can also use the `@smartescrow` or `@smart-escrow` tags)

## Historical WASM Devnets

### Devnet5 - 2025-08-04 Release

The WASM Devnet (Smart Escrows) has been updated to its fifth monthly release.

The `rippled` code can be found in this branch: https://github.com/XRPLF/rippled/tree/ripple/smart-escrow

Commit: 58741d2

We are nearing feature-complete / code freeze. Please [open an issue](https://github.com/ripple/craft/issues) to communicate any requests to the development team.

Devnet details:

- URL: wasm.devnet.rippletest.net (port 51234 for HTTP, 51233 for WS)
- Explorer: https://custom.xrpl.org/wasm.devnet.rippletest.net
- Faucet: https://wasmfaucet.devnet.rippletest.net/accounts

`rippled` Changelog:

- Charge appropriate gas costs for host functions
- Change the return type of the `finish` function from a `bool` to an `int` (any value greater than 0 releases the escrow)
  - The return value is displayed in the metadata, making debugging easier: developers can use different return codes to mean different things (such as different kinds of errors)
- Add more host functions
  - More keylets
  - NFT utils (e.g. getting the issuer from the NFT ID)
  - Check whether an amendment is enabled
  - Check a public key signature
  - Fetch additional ledger header info
  - A series of functions to perform operations on floats (e.g. XRP/IOU/MPT numbers)
- Better error handling (e.g. `ComputationAllowance` of 0)
- Various bugfixes

Craft Changelog:

- Support the new `rippled` features mentioned above
- Lots of additional documentation

Tooling:

- Writing Rust WASM extensions: https://github.com/ripple/craft
- Python: xrpl-py v4.4.0b0
- JS: `xrpl@4.5.0-smartescrow.0`, `ripple-binary-codec@2.6.0-smartescrow.0` (you can also use the `@smartescrow` or `@smart-escrow` tags)

### Devnet4 - 2025-07-03 Release

The Programmability Devnet has been updated to its fourth monthly release.

Devnet details:

URL: wasm.devnet.rippletest.net (port 51234 for HTTP, 51233 for WS)

Explorer: https://custom.xrpl.org/wasm.devnet.rippletest.net

Faucet: https://wasmfaucet.devnet.rippletest.net/accounts

Changelog:

- `EscrowCreate` will now prevent you from uploading bad WASM code.
- A fee schedule skeleton was added.
- Misc performance fixes in the host functions.

Craft Changelog:

- Better Docker support
- Switch from using WasmEdge to WAMR (to match rippled)

Tooling:

- Writing Rust WASM extensions: https://github.com/ripple/craft
- Python: xrpl-py v4.3.0b0
- JS: xrpl@4.4.0-smartescrow.0, ripple-binary-codec@2.5.0-smartescrow.0 (you can also use the @smartescrow or @smart-escrow tags)

The rippled commit hash is: 65b0b976d98e54226136fad8e733d17b7fbb511e

### Devnet3 - 2025-06-04 Release

The Programmability Devnet has been updated to its third monthly release.

Devnet details:

URL: wasm.devnet.rippletest.net (port 51234 for HTTP, 51233 for WS)

Explorer: https://custom.xrpl.org/wasm.devnet.rippletest.net

Faucet: https://wasmfaucet.devnet.rippletest.net/accounts

Changelog:

- The way memory management works with regard to host functions has been changed significantly. The set of host functions has changed a lot as a result. This has been reflected in changes to XLS-102d: https://github.com/XRPLF/XRPL-Standards/discussions/279
- Some of the latest features that were recently merged to develop have been added in and enabled (Batch, Permission Delegation).
- The name of the function in the FinishFunction has switched from `ready` to `finish` (this made more sense to us).
- A `GasUsed` parameter has been added to the metadata in a transaction. When coupled with `simulate` RPC, this means that you can estimate gas costs before the transaction is submitted.
- Some rippled logging changes.

**Tooling:**
- Writing Rust WASM extensions: https://github.com/ripple/craft
- Python: xrpl-py v4.2.0b3
- JS: xrpl@4.3.0-smartescrow.3, ripple-binary-codec@2.4.0-smartescrow.2 (you can also use the @smartescrow or @smart-escrow tags)

### Devnet2 - 2025-05-01 Release

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
