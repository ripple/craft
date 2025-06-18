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

  // Notary escrow?
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

  const response2 = await submit({
    TransactionType: 'EscrowCreate',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: "100000",
    Destination: wallet.address,
    CancelAfter: close_time + 20,
    FinishAfter: close_time + 2,
    FinishFunction: finish,
  }, masterWallet)
  // console.log(JSON.stringify(response2.result, null, 4))
  if (response2.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1);
  const sequence = response2.result.tx_json.Sequence

  await sleep(5000)
  await client.request({command: 'ledger_accept'})

  const response3 = await submit({
    TransactionType: 'EscrowFinish',
    Account: wallet.address,
    Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    OfferSequence: sequence,
    ComputationAllowance: 1000000,
  }, wallet)
  // console.log(JSON.stringify(response3.result, null, 4))

  const response4 = await submit({
    TransactionType: 'EscrowFinish',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    OfferSequence: sequence,
    ComputationAllowance: 1000000,
  }, masterWallet)

  await client.disconnect()
}

test()
