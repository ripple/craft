mod vm;

use std::collections::HashMap;
use wasmedge_sdk::{wasi::WasiModule, Module, Store, Vm};
use crate::vm::run_func;
use clap::Parser;

/// WasmEdge WASM testing utility
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the WASM file
    #[arg(short, long)]
    wasm_file: String,

    /// Function to execute
    #[arg(short, long, default_value = "compare_accountID")]
    function: String,
}

fn main() {
    let args = Args::parse();

    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let wasm_module = match Module::from_file(None, &args.wasm_file) {
        Ok(module) => module,
        Err(e) => {
            eprintln!("Failed to load WASM module from {}: {}", args.wasm_file, e);
            return;
        }
    };
    
    vm.register_module(None, wasm_module.clone()).unwrap();

    let escrow_tx_json_str =  r#"{
       "Account" : "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
       "Fee" : "10",
       "Flags" : 2147483648,
       "OfferSequence" : 2,
       "Owner" : "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
       "Sequence" : 3,
       "SigningPubKey" : "0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020",
       "TransactionType" : "EscrowFinish",
       "TxnSignature" : "30450221008AD5EE48F7F1047813E79C174FE401D023A4B4A7B99AF826E081DB1DFF7B9C510220133F05B7FD3D7D7F163E8C77EE0A49D02619AB6C77CC3487D0095C9B34033C1C",
       "hash" : "74465121372813CBA4C77E31F12E137163F5B2509B16AC1703ECF0DA194B2DD4"
   }"#;

    let escrow_lo_json_str = r#"{
       "Account" : "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
       "Amount" : "100000",
       "CancelAfter" : 790297421,
       "Destination" : "rBYn44yhs8cf8G2t79XMUHYQpp2ayhqwcw",
       "DestinationNode" : "0",
       "FinishAfter" : 790297403,
       "FinishFunction" : "0061736D0100000001180460027F7F0060017F017F60027F7F017F60047F7F7F7F00030C0B01010200000000000003000405017001030305030100110619037F01418080C0000B7F0041DD85C0000B7F0041E085C0000B074205066D656D6F7279020008616C6C6F6361746500000F636865636B5F6163636F756E74494400020A5F5F646174615F656E6403010B5F5F686561705F6261736503020908010041010B02060A0AF5360B610002",
       "Flags" : 0,
       "LedgerEntryType" : "Escrow",
       "OwnerNode" : "0",
       "PreviousTxnID" : "CF25D1C6B8E637C7DAC61B586F820A16896A3090D9F6FBF9FA00D8B13A265647",
       "PreviousTxnLgrSeq" : 4,
       "index" : "9BC6631F3EC761CF9BD846D006560E2D57B0A5C91D4570AEB209645B189A702F"
    }"#;

    match run_func(&mut vm, &args.function, escrow_tx_json_str.as_bytes().to_vec(), escrow_lo_json_str.as_bytes().to_vec()) {
        Ok(result) => println!("Function result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

