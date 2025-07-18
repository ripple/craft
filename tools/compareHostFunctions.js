if (process.argv.length !== 3) {
  console.error(
    'Usage: ' +
      process.argv[0] +
      ' ' +
      process.argv[1] +
      ' path/to/rippled',
  )
  process.exit(1)
}

////////////////////////////////////////////////////////////////////////
//  Get all necessary files from rippled
////////////////////////////////////////////////////////////////////////
const path = require('path')
const fs = require('fs/promises')

async function readFileFromGitHub(repo, filename) {
    if (!repo.includes('tree')) {
        repo += '/tree/HEAD'
    }
    let url = repo.replace('github.com', 'raw.githubusercontent.com')
    url = url.replace('tree/', '')
    url += '/' + filename

    if (!url.startsWith('http')) {
        url = 'https://' + url
    }

    try {
        const response = await fetch(url)
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`)
        }
        return await response.text()
    } catch (e) {
        console.error(`Error reading ${url}: ${e.message}`)
        process.exit(1)
    }
}

async function readFile(folder, filename) {
    const filePath = path.join(folder, filename)
    try {
        return await fs.readFile(filePath, 'utf-8')
    } catch (e) {
        throw new Error(`File not found: ${filePath}, ${e.message}`)
    }
}

const read = process.argv[2].includes('github.com')
    ? readFileFromGitHub
    : readFile

async function main() {
    const wasmImportFile = await read(
        process.argv[2], 'src/xrpld/app/misc/WasmVM.cpp',
    )
    const hostWrapperFile = await read(
        process.argv[2], 'src/xrpld/app/misc/WasmHostFuncWrapper.h',
    )
    const rustHostFunctionFile = await readFile(__dirname, '../xrpl-std/src/host/host_bindings.rs')
    const rustHostFunctionTestFile = await readFile(__dirname, '../xrpl-std/src/host/host_bindings_for_testing.rs')

    let importHits = [
    ...wasmImportFile.matchAll(
        /^ *WASM_IMPORT_FUNC2? *\(i, *([A-Za-z0-9]+), *("([A-Za-z0-9_]+)",)? *hfs, *[0-9]+\);$/gm,
    ),
    ]
    const imports = importHits.map((hit) => [hit[1], hit[3] != null ? hit[3] : hit[1]]).filter(
    (hit) => hit[0] !== 'getLedgerSqnOld')

    let wrapperHits = [
    ...hostWrapperFile.matchAll(
        /^ *using ([A-Za-z0-9]+)_proto =[ \n]*([A-Za-z0-9_]+)\(([A-Za-z0-9_\* \n,]+)\);$/gm,
    ),
    ]
    const wrappers = wrapperHits.map((hit) => [hit[1], hit[2], hit[3].split(',').map((s) => s.trim())])
    if (imports.length !== wrappers.length) {
    console.error(
        'Imports and Host Functions do not match in length! ' +
        imports.length +
        ' !== ' +
        wrappers.length,
    )
    process.exit(1)
    }

    for (let i = 0; i < imports.length; i++) {
    if (imports[i][0] !== wrappers[i][0]) {
        console.error(
        'Imports and Host Functions do not match at index ' +
            i +
            ': ' +
            imports[i][0] +
            ' !== ' +
            wrappers[i][0],
        )
        process.exit(1)
    }
    }

    const cppHostFunctions = imports.map((hit, i) => {
    return {
        name: hit[1],
        return: wrappers[i][1],
        params: wrappers[i][2],
    }
    })

    const paramTranslation = {
        'i32': 'int32_t',
        'u32': 'uint32_t',
        'usize': 'int32_t',
        'i64': 'int64_t',
        '*const u8': 'uint8_t const*',
        '*mut u8': 'uint8_t*',
    }

    function translateParamType(param) {
    if (param in paramTranslation) {
        return paramTranslation[param]
    }
    console.error(`Unknown parameter type: ${param}`)
    process.exit(1)
    }

    for (let file of [rustHostFunctionFile, rustHostFunctionTestFile]) {
        let rustHits = [
            ...file.matchAll(
                /^ *pub (unsafe )?fn ([A-Za-z0-9_]+)\([ \n]*([A-Za-z0-9_:*, \n]+)\) -> ([A-Za-z0-9]+)/gm,
            ),
        ]
        const rustFuncs = rustHits.map((hit) => [hit[2], hit[4], hit[3].trim().split(',').map((s) => s.trim()).filter((s) => s.length > 0).map((s) => s.split(':')[1].trim())])
        const rustHostFunctions = rustFuncs.map((hit) => {
            return {
                name: hit[0],
                return: translateParamType(hit[1]),
                params: hit[2].map(translateParamType),
            }
        })

        if (rustHostFunctions.length !== cppHostFunctions.length) {
            console.error(
                'Rust Host Functions and C++ Host Functions do not match in length! ' +
                rustHostFunctions.length +
                ' !== ' + cppHostFunctions.length
            )
            if (rustHostFunctions.length < cppHostFunctions.length) {
                const missing = cppHostFunctions.filter(f => !rustHostFunctions.some(rf => rf.name === f.name))
                console.error('Missing Rust Host Functions:', missing.map(f => f.name).join(', '))
            } else {
                const missing = rustHostFunctions.filter(f => !cppHostFunctions.some(rf => rf.name === f.name))
                console.error('Missing C++ Host Functions:', missing.map(f => f.name).join(', '))
            }
            process.exit(1)
        }

        let hasError = false
        rustHostFunctions.forEach((hit, index) => {
            if (hit.name !== cppHostFunctions[index].name) {
                console.error(
                    `Rust Host Function name mismatch: ${hit.name} !== ${cppHostFunctions[index].name}`,
                )
                hasError = true
            }
            else if (hit.return !== cppHostFunctions[index].return) {
                console.error(
                    `Rust Host Function return type mismatch for ${hit.name}: ${hit.return} !== ${cppHostFunctions[index].return}`,
                )
                hasError = true
            }
            else if (hit.params.length !== cppHostFunctions[index].params.length) {
                console.error(
                    `Rust Host Function parameter count mismatch for ${hit.name}: ${hit.params.length} !== ${cppHostFunctions[index].params.length}`,
                )
                hasError = true
            } else {
                hit.params.forEach((param, paramIndex) => {
                    if (param !== cppHostFunctions[index].params[paramIndex]) {
                        console.error(
                            `Rust Host Function parameter type mismatch for ${hit.name}, parameter ${paramIndex}: ${param} !== ${cppHostFunctions[index].params[paramIndex]}`,
                        )
                        hasError = true
                    }
                })
            }
        })
        if (hasError) {
        process.exit(1)
        }
    }

    console.log('All host functions match between Rust and C++ implementations.')
}

main()
