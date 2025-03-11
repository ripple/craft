const xrpl = require("xrpl");

const sleep = ms => new Promise(r => setTimeout(r, ms));


const client = new xrpl.Client("ws://127.0.0.1:6006");

async function submit(tx, seed) {

    // Add network_id: 63456 to the tx_json
    tx.NetworkID = 63456


    const result = await client.request({
        command: "submit",
        tx_json: tx,
        secret: seed,
    })
    console.log("SUBMITTED " + tx.TransactionType)
    console.log(result.result ?? result)

    await client.request({command: 'ledger_accept'})

    // now there is tx_json. why?
    // depends on api_version!
    return client.request({command: 'tx', transaction: result.result.tx_json.hash})
    // return client.request({command: 'tx', transaction: result.result.hash}) // no tx_json here
}

async function test() {
  await client.connect();
  const wallet = xrpl.Wallet.generate();
  console.log("connected");
  seed = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb"

  await submit({
    TransactionType: 'Payment',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: xrpl.xrpToDrops(1000),
    Destination: wallet.address,
  }, seed)

  const close_time = (
    await client.request({
      command: 'ledger',
      ledger_index: 'validated',
    })
  ).result.ledger.close_time

  // FinishFunction which requires ledger sequence of 5
  const finish = "0061736d010000000105016000017f021b0108686f73745f6c69620e6765745f6c65646765725f73716e0000030201000405017001010105030100100619037f01418080c0000b7f00418080c0000b7f00418080c0000b072d04066d656d6f7279020005726561647900010a5f5f646174615f656e6403010b5f5f686561705f6261736503020a0d010b0010808080800041044a0b0073046e616d6500100f6c65646765725f73716e2e7761736d014602003c5f5a4e31306c65646765725f73716e38686f73745f6c696231346765745f6c65646765725f73716e313768666335646562363031363564626438614501057265616479071201000f5f5f737461636b5f706f696e746572004d0970726f64756365727302086c616e6775616765010452757374000c70726f6365737365642d6279010572757374631d312e38312e30202865656239306364613120323032342d30392d303429002c0f7461726765745f6665617475726573022b0f6d757461626c652d676c6f62616c732b087369676e2d657874"

  console.log(finish, finish.length)


  try {
    console.log("Starting")
    const response2 = await submit({
      TransactionType: 'EscrowCreate',
      Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      Amount: "100000",
      Destination: wallet.address,
      CancelAfter: close_time + 20,
      FinishAfter: close_time + 2,
      FinishFunction: finish,
    }, seed)
    console.log(JSON.stringify(response2.result, null, 4))
    if (response2.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1);
    // const sequence = response2.result.tx_json.Sequence // need tx_json here
    
    // due to api_version, no tx_json here
    const sequence = response2.result.Sequence
    

    await sleep(5000)
    console.log("Ledger Accept")
    await client.request({command: 'ledger_accept'})

    const response3 = await submit({
      TransactionType: 'EscrowFinish',
      Account: wallet.address,
      Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      OfferSequence: sequence,
    }, wallet.seed)
    // console.log(JSON.stringify(response3.result, null, 4))

    // {
    //   deprecated: "Signing support in the 'submit' command has been deprecated and will be removed in a future version of the server. Please migrate to a standalone signing tool.",
    //   engine_result: 'tecNO_PERMISSION',
    //   engine_result_code: 139,
    //   engine_result_message: 'No permission to perform requested operation.',
    //   hash: '16D2FEC8D899B4248CF60CDFFDE05F443B237D7EDEF14CF523DFFC4E74E1CA57',
    //   tx_blob: '1200022280000000240000000320190000000268400000000000000A7321EDEE2C0CEA7894FEC8505922F99DE177DF1A2908BF048E7850008EFE49A2CD17E87440E1B3DE9A6D3DB4E14233D5BBF60EF68A243A0F064C4EE2DAF2B3FB26DDF52E7656DD805FAA1F3FE5C4CFCFFEC807356292C73B66F3BEA7D38421DB11C01C090A81149D47AB25FDF231305587F2DDE52283857D6C8BEF8214B5F762798A53D543A014CAF8B297CFF8F2F937E8',
    //   tx_json: {
    //     Account: 'rELcAjex29bfBCS3TRBubAWH7WTTrob3Yr',
    //     Fee: '10',
    //     Flags: 2147483648,
    //     OfferSequence: 2,
    //     Owner: 'rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh',
    //     Sequence: 3,
    //     SigningPubKey: 'EDEE2C0CEA7894FEC8505922F99DE177DF1A2908BF048E7850008EFE49A2CD17E8',
    //     TransactionType: 'EscrowFinish',
    //     TxnSignature: 'E1B3DE9A6D3DB4E14233D5BBF60EF68A243A0F064C4EE2DAF2B3FB26DDF52E7656DD805FAA1F3FE5C4CFCFFEC807356292C73B66F3BEA7D38421DB11C01C090A'
    //   }
    // }
    

    const response4 = await submit({
      TransactionType: 'EscrowFinish',
      Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      OfferSequence: sequence,
    }, seed)

    // {
    //   deprecated: "Signing support in the 'submit' command has been deprecated and will be removed in a future version of the server. Please migrate to a standalone signing tool.",
    //   engine_result: 'tesSUCCESS',
    //   engine_result_code: 0,
    //   engine_result_message: 'The transaction was applied. Only final in a validated ledger.',
    //   hash: '74465121372813CBA4C77E31F12E137163F5B2509B16AC1703ECF0DA194B2DD4',
    //   tx_blob: '1200022280000000240000000320190000000268400000000000000A73210330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020744730450221008AD5EE48F7F1047813E79C174FE401D023A4B4A7B99AF826E081DB1DFF7B9C510220133F05B7FD3D7D7F163E8C77EE0A49D02619AB6C77CC3487D0095C9B34033C1C8114B5F762798A53D543A014CAF8B297CFF8F2F937E88214B5F762798A53D543A014CAF8B297CFF8F2F937E8',
    //   tx_json: {
    //     Account: 'rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh',
    //     Fee: '10',
    //     Flags: 2147483648,
    //     OfferSequence: 2,
    //     Owner: 'rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh',
    //     Sequence: 3,
    //     SigningPubKey: '0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020',
    //     TransactionType: 'EscrowFinish',
    //     TxnSignature: '30450221008AD5EE48F7F1047813E79C174FE401D023A4B4A7B99AF826E081DB1DFF7B9C510220133F05B7FD3D7D7F163E8C77EE0A49D02619AB6C77CC3487D0095C9B34033C1C'
    //   }
    // }

  } catch (e) {
    console.log(e)
  }

  

  await client.disconnect()
}

test()









// 1. building faster -- build on a bigger remote AWS machine?
// 2. backward incompatibiliy.