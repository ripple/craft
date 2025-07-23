const path = require('path')
const fs = require('fs')

function main() {
    const srcPath = path.resolve(__dirname, `./target/wasm32-unknown-unknown/release/codecov_tests.wasm`)
    const data = fs.readFileSync(srcPath)
    const wasm = data.toString('hex')
    
    const dstPath = path.resolve(__dirname, '../../../rippled-all/smart-escrows/src/test/app/wasm_fixtures/fixtures.cpp')
    const dstContent = fs.readFileSync(dstPath, 'utf8')
    const updatedContent = dstContent.replace(/extern std::string const codecovWasm = "[^;]*;/, `extern std::string const codecovWasm = "${wasm}";`)
    fs.writeFileSync(dstPath, updatedContent)
}

main()
