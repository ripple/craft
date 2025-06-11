const fs = require('fs'); // Only in Node.js
const wasmBuffer = fs.readFileSync('../../projects/ledger_sqn/target/wasm32-unknown-unknown/release/ledger_sqn.wasm');

const importObject = {
    host_lib: {
      getLedgerSqn: () => {
        console.log("getLedgerSqn called!");
        return 123456; // Replace with your own logic
      },
    },
  };

WebAssembly.instantiate(wasmBuffer, importObject).then(({ instance }) => {
    // Call your exported function here
    const result = instance.exports.ready(); // Pass args as needed
    console.log("Result:", result);
});

