const xrpl = require("xrpl");

const sleep = ms => new Promise(r => setTimeout(r, ms));


const client = new xrpl.Client("ws://127.0.0.1:6006");

async function submit(tx, wallet) {
    const result = await client.submit(tx, { wallet })

    console.log("SUBMITTED " + tx.TransactionType)
    console.log(result.result ?? result)

    await client.request({command: 'ledger_accept'})

    return client.request({command: 'tx', transaction: result.result.tx_json.hash})
}

async function test() {
  await client.connect();
  const wallet = xrpl.Wallet.generate();
  console.log("connected");
  const masterSeed = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb"
  const masterWallet = xrpl.Wallet.fromSeed(masterSeed, { algorithm: xrpl.ECDSA.secp256k1});

  await submit({
    TransactionType: 'Payment',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: xrpl.xrpToDrops(1000),
    Destination: wallet.address,
  }, masterWallet)

  const close_time = (
    await client.request({
      command: 'ledger',
      ledger_index: 'validated',
    })
  ).result.ledger.close_time

  // FinishFunction which requires ledger sequence of 5
  const finish = '0061736d010000000105016000017f02190108686f73745f6c69620c6765' +
        '744c656467657253716e00000302010005030100100611027f00418080c0' +
        '000b7f00418080c0000b072e04066d656d6f727902000666696e69736800' +
        '010a5f5f646174615f656e6403000b5f5f686561705f6261736503010a09' +
        '010700100041044a0b004d0970726f64756365727302086c616e67756167' +
        '65010452757374000c70726f6365737365642d6279010572757374631d31' +
        '2e38352e31202834656231363132353020323032352d30332d3135290049' +
        '0f7461726765745f6665617475726573042b0f6d757461626c652d676c6f' +
        '62616c732b087369676e2d6578742b0f7265666572656e63652d74797065' +
        '732b0a6d756c746976616c7565'

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
    }, masterWallet)
    console.log(JSON.stringify(response2.result, null, 4))
    if (response2.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1);
    // const sequence = response2.result.tx_json.Sequence // need tx_json here
    
    // due to api_version, no tx_json here
    const sequence = response2.result.tx_json.Sequence
    

    await sleep(5000)
    console.log("Ledger Accept")
    await client.request({command: 'ledger_accept'})

    const response3 = await submit({
      TransactionType: 'EscrowFinish',
      Account: wallet.address,
      Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
      OfferSequence: sequence,
      ComputationAllowance: 5,
    }, wallet)
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
    //     ComputationAllowance: '1000000',
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
      ComputationAllowance: 5,
    }, masterWallet)

    // {
    //   deprecated: "Signing support in the 'submit' command has been deprecated and will be removed in a future version of the server. Please migrate to a standalone signing tool.",
    //   engine_result: 'tesSUCCESS',
    //   engine_result_code: 0,
    //   engine_result_message: 'The transaction was applied. Only final in a validated ledger.',
    //   hash: '74465121372813CBA4C77E31F12E137163F5B2509B16AC1703ECF0DA194B2DD4',
    //   tx_blob: '1200022280000000240000000320190000000268400000000000000A73210330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020744730450221008AD5EE48F7F1047813E79C174FE401D023A4B4A7B99AF826E081DB1DFF7B9C510220133F05B7FD3D7D7F163E8C77EE0A49D02619AB6C77CC3487D0095C9B34033C1C8114B5F762798A53D543A014CAF8B297CFF8F2F937E88214B5F762798A53D543A014CAF8B297CFF8F2F937E8',
    //   tx_json: {
    //     Account: 'rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh',
    //     ComputationAllowance: '1000000',
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
