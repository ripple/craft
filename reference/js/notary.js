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

  // fund our new wallet with 1000 test XRP
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

  // Notary escrow FinishFunction
  const finish = "0061736D0100000001210460037F7F7F017F60087F7F7F7F7F7F7F7F017F60057F7F7F7F7F" +
    "017F6000017F02570308686F73745F6C69621C6765745F63757272656E745F6C6564676572" +
    "5F6F626A5F6669656C64000008686F73745F6C69621163726564656E7469616C5F6B65796C" +
    "6574000108686F73745F6C696205747261636500020302010305030100110619037F014180" +
    "80C0000B7F0041A380C0000B7F0041B080C0000B072E04066D656D6F727902000666696E69" +
    "736800030A5F5F646174615F656E6403010B5F5F686561705F6261736503020A8C02018902" +
    "01057F230041E0006B22002400200041D0006A22014100360200200041C8006A2202420037" +
    "030020004200370340027F024041818020200041406B22034114100041004A044020004118" +
    "6A2001280200360200200041106A200229030037030020002000290340370308200041D800" +
    "6A22044200370300200142003703002002420037030020004200370340200041086A220141" +
    "1420014114418080C000411220034120100141004A0D010B41000C010B200041386A200429" +
    "0300370300200041306A200041D0006A290300370300200041286A200041C8006A29030037" +
    "030020002000290340370320419280C0004111200041206A4120410110021A41010B200041" +
    "E0006A24000B0B2C0100418080C0000B237465726D73616E64636F6E646974696F6E734372" +
    "6564656E7469616C204B65796C6574004D0970726F64756365727302086C616E6775616765" +
    "010452757374000C70726F6365737365642D6279010572757374631D312E38352E31202834" +
    "656231363132353020323032352D30332D31352900490F7461726765745F66656174757265" +
    "73042B0F6D757461626C652D676C6F62616C732B087369676E2D6578742B0F726566657265" +
    "6E63652D74797065732B0A6D756C746976616C7565"

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
      ComputationAllowance: 1000000,
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
      ComputationAllowance: 1000000,
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
