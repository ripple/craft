const xrpl = require("xrpl");

const sleep = ms => new Promise(r => setTimeout(r, ms));


const client = new xrpl.Client("ws://127.0.0.1:6006");

async function submit(tx, seed) {
    const result = await client.request({
        command: "submit",
        tx_json: tx,
        secret: seed,
    })
    console.log("SUBMITTED " + tx.TransactionType)
    console.log(result.result ?? result)

    await client.request({command: 'ledger_accept'})

    return client.request({command: 'tx', transaction: result.result.tx_json.hash})
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

  const finish = "0061736d0100000001180460027f7f0060017f017f60027f7f017f60047f7f7f7f" +
            "00030c0b01010200000000000003000405017001030305030100110619037f0141" +
            "8080c0000b7f0041dd85c0000b7f0041e085c0000b074205066d656d6f72790200" +
            "08616c6c6f6361746500000f636865636b5f6163636f756e74494400020a5f5f64" +
            "6174615f656e6403010b5f5f686561705f6261736503020908010041010b02060a" +
            "0af5360b61000240200041004e0440200045044041010f0b418882c0002d00001a" +
            "200010012200450d0120000f0b230041206b220024002000410036021820004101" +
            "36020c200041b480c00036020820004204370210200041086a41d080c000100500" +
            "0b000bee2202087f017e02400240024002400240024002400240200041f4014d04" +
            "4041ac85c000280200220241102000410b6a41f803712000410b491b2205410376" +
            "22007622014103710d01200541b485c0002802004d0d0720010d0241b085c00028" +
            "020022000d030c070b2000410b6a2201417871210541b085c0002802002208450d" +
            "06411f2107410020056b2103200041f4ffff074d04402005410620014108766722" +
            "006b7641017120004101746b413e6a21070b2007410274419482c0006a28020022" +
            "0245044041002100410021010c040b4100210020054100411920074101766b2007" +
            "411f461b7421044100210103400240200228020441787122062005490d00200620" +
            "056b220620034f0d0020022101200622030d0041002103200221000c060b200228" +
            "021422062000200620022004411d764104716a41106a2802002202471b20002006" +
            "1b21002004410174210420020d000b0c030b02402001417f7341017120006a2206" +
            "410374220041a483c0006a2203200041ac83c0006a280200220128020822044704" +
            "402004200336020c200320043602080c010b41ac85c0002002417e200677713602" +
            "000b20012000410372360204200020016a220020002802044101723602040c060b" +
            "024041022000742203410020036b72200120007471682206410374220041a483c0" +
            "006a2203200041ac83c0006a280200220128020822044704402004200336020c20" +
            "0320043602080c010b41ac85c0002002417e200677713602000b20012005410372" +
            "360204200120056a2206200020056b2204410172360204200020016a2004360200" +
            "41b485c00028020022020440200241787141a483c0006a210041bc85c000280200" +
            "2103027f41ac85c0002802002205410120024103767422027145044041ac85c000" +
            "200220057236020020000c010b20002802080b2102200020033602082002200336" +
            "020c2003200036020c200320023602080b41bc85c000200636020041b485c00020" +
            "043602000c050b200068410274419482c0006a280200220128020441787120056b" +
            "2103200121020240034002400240200128021022000d00200128021422000d0020" +
            "022802182107024002402002200228020c22004604402002411441102002280214" +
            "22001b6a28020022010d01410021000c020b20022802082201200036020c200020" +
            "013602080c010b200241146a200241106a20001b21040340200421062001220041" +
            "146a200041106a200028021422011b210420004114411020011b6a28020022010d" +
            "000b200641003602000b2007450d032002200228021c410274419482c0006a2201" +
            "28020047044020074110411420072802102002461b6a20003602002000450d040c" +
            "020b2001200036020020000d0141b085c00041b085c000280200417e200228021c" +
            "77713602000c030b200028020441787120056b22012003200120034922011b2103" +
            "2000200220011b2102200021010c010b0b20002007360218200228021022010440" +
            "20002001360210200120003602180b20022802142201450d002000200136021420" +
            "0120003602180b02400240200341104f044020022005410372360204200220056a" +
            "22062003410172360204200320066a200336020041b485c0002802002204450d01" +
            "200441787141a483c0006a210041bc85c0002802002101027f41ac85c000280200" +
            "2205410120044103767422047145044041ac85c000200420057236020020000c01" +
            "0b20002802080b2104200020013602082004200136020c2001200036020c200120" +
            "043602080c010b2002200320056a2200410372360204200020026a220020002802" +
            "044101723602040c010b41bc85c000200636020041b485c00020033602000b2002" +
            "41086a0f0b20002001724504404100210141022007742200410020006b72200871" +
            "2200450d03200068410274419482c0006a28020021000b2000450d010b03402000" +
            "20012000280204417871220420056b220620034922071b21082000280210220245" +
            "0440200028021421020b20012008200420054922001b210120032006200320071b" +
            "20001b2103200222000d000b0b2001450d00200541b485c00028020022004d2003" +
            "200020056b4f710d0020012802182107024002402001200128020c220046044020" +
            "0141144110200128021422001b6a28020022020d01410021000c020b2001280208" +
            "2202200036020c200020023602080c010b200141146a200141106a20001b210403" +
            "40200421062002220041146a200041106a200028021422021b2104200041144110" +
            "20021b6a28020022020d000b200641003602000b02402007450d00024020012001" +
            "28021c410274419482c0006a220228020047044020074110411420072802102001" +
            "461b6a20003602002000450d020c010b2002200036020020000d0041b085c00041" +
            "b085c000280200417e200128021c77713602000c010b2000200736021820012802" +
            "102202044020002002360210200220003602180b20012802142202450d00200020" +
            "02360214200220003602180b0240200341104f0440200120054103723602042001" +
            "20056a22022003410172360204200220036a200336020020034180024f04402002" +
            "200310070c020b200341f8017141a483c0006a2100027f41ac85c0002802002204" +
            "410120034103767422037145044041ac85c000200320047236020020000c010b20" +
            "002802080b2103200020023602082003200236020c2002200036020c2002200336" +
            "02080c010b2001200320056a2200410372360204200020016a2200200028020441" +
            "01723602040b0c010b024002400240024002400240200541b485c0002802002201" +
            "4b0440200541b885c00028020022004f044041002100200541af80046a22014110" +
            "7640002202417f4622030d0720024110742202450d0741c485c000410020014180" +
            "807c7120031b220441c485c0002802006a220036020041c885c00041c885c00028" +
            "02002201200020002001491b3602000240024041c085c000280200220304404194" +
            "83c0002100034020002802002201200028020422066a2002460d02200028020822" +
            "000d000b0c020b41d085c00028020022004100200020024d1b45044041d085c000" +
            "20023602000b41d485c00041ff1f360200419883c0002004360200419483c00020" +
            "0236020041b083c00041a483c00036020041b883c00041ac83c00036020041ac83" +
            "c00041a483c00036020041c083c00041b483c00036020041b483c00041ac83c000" +
            "36020041c883c00041bc83c00036020041bc83c00041b483c00036020041d083c0" +
            "0041c483c00036020041c483c00041bc83c00036020041d883c00041cc83c00036" +
            "020041cc83c00041c483c00036020041e083c00041d483c00036020041d483c000" +
            "41cc83c00036020041e883c00041dc83c00036020041dc83c00041d483c0003602" +
            "0041a083c000410036020041f083c00041e483c00036020041e483c00041dc83c0" +
            "0036020041ec83c00041e483c00036020041f883c00041ec83c00036020041f483" +
            "c00041ec83c000360200418084c00041f483c00036020041fc83c00041f483c000" +
            "360200418884c00041fc83c000360200418484c00041fc83c000360200419084c0" +
            "00418484c000360200418c84c000418484c000360200419884c000418c84c00036" +
            "0200419484c000418c84c00036020041a084c000419484c000360200419c84c000" +
            "419484c00036020041a884c000419c84c00036020041a484c000419c84c0003602" +
            "0041b084c00041a484c00036020041b884c00041ac84c00036020041ac84c00041" +
            "a484c00036020041c084c00041b484c00036020041b484c00041ac84c000360200" +
            "41c884c00041bc84c00036020041bc84c00041b484c00036020041d084c00041c4" +
            "84c00036020041c484c00041bc84c00036020041d884c00041cc84c00036020041" +
            "cc84c00041c484c00036020041e084c00041d484c00036020041d484c00041cc84" +
            "c00036020041e884c00041dc84c00036020041dc84c00041d484c00036020041f0" +
            "84c00041e484c00036020041e484c00041dc84c00036020041f884c00041ec84c0" +
            "0036020041ec84c00041e484c000360200418085c00041f484c00036020041f484" +
            "c00041ec84c000360200418885c00041fc84c00036020041fc84c00041f484c000" +
            "360200419085c000418485c000360200418485c00041fc84c000360200419885c0" +
            "00418c85c000360200418c85c000418485c00036020041a085c000419485c00036" +
            "0200419485c000418c85c00036020041a885c000419c85c000360200419c85c000" +
            "419485c00036020041c085c000200236020041a485c000419c85c00036020041b8" +
            "85c000200441286b220036020020022000410172360204200020026a4128360204" +
            "41cc85c00041808080013602000c080b200220034d200120034b720d0020002802" +
            "0c450d030b41d085c00041d085c0002802002200200220002002491b3602002002" +
            "20046a2101419483c0002100024002400340200120002802002206470440200028" +
            "020822000d010c020b0b200028020c450d010b419483c000210003400240200320" +
            "0028020022014f04402003200120002802046a2206490d010b200028020821000c" +
            "010b0b41c085c000200236020041b885c000200441286b22003602002002200041" +
            "0172360204200020026a412836020441cc85c00041808080013602002003200641" +
            "206b41787141086b22002000200341106a491b2201411b360204419483c0002902" +
            "002109200141106a419c83c00029020037020020012009370208419883c0002004" +
            "360200419483c0002002360200419c83c000200141086a36020041a083c0004100" +
            "3602002001411c6a2100034020004107360200200041046a22002006490d000b20" +
            "012003460d0720012001280204417e713602042003200120036b22004101723602" +
            "042001200036020020004180024f04402003200010070c080b200041f8017141a4" +
            "83c0006a2101027f41ac85c0002802002202410120004103767422007145044041" +
            "ac85c000200020027236020020010c010b20012802080b21002001200336020820" +
            "00200336020c2003200136020c200320003602080c070b20002002360200200020" +
            "0028020420046a360204200220054103723602042006410f6a41787141086b2201" +
            "200220056a22046b2103200141c085c000280200460d03200141bc85c000280200" +
            "460d04200128020422054103714101460440200120054178712200100820002001" +
            "6a22012802042105200020036a21030b20012005417e7136020420042003410172" +
            "360204200320046a200336020020034180024f04402004200310070c060b200341" +
            "f8017141a483c0006a2100027f41ac85c000280200220141012003410376742203" +
            "7145044041ac85c000200120037236020020000c010b20002802080b2103200020" +
            "043602082003200436020c2004200036020c200420033602080c050b41b885c000" +
            "200020056b220136020041c085c00041c085c000280200220020056a2202360200" +
            "2002200141017236020420002005410372360204200041086a21000c060b41bc85" +
            "c00028020021000240200120056b2202410f4d044041bc85c000410036020041b4" +
            "85c000410036020020002001410372360204200020016a22012001280204410172" +
            "3602040c010b41b485c000200236020041bc85c000200020056a22033602002003" +
            "2002410172360204200020016a2002360200200020054103723602040b20004108" +
            "6a0f0b2000200420066a36020441c085c00041c085c0002802002200410f6a4178" +
            "71220141086b220236020041b885c00041b885c00028020020046a220320002001" +
            "6b6a41086a220136020020022001410172360204200020036a412836020441cc85" +
            "c00041808080013602000c030b41c085c000200436020041b885c00041b885c000" +
            "28020020036a2200360200200420004101723602040c010b41bc85c00020043602" +
            "0041b485c00041b485c00028020020036a22003602002004200041017236020420" +
            "0020046a20003602000b200241086a0f0b4100210041b885c00028020022012005" +
            "4d0d0041b885c000200120056b220136020041c085c00041c085c0002802002200" +
            "20056a22023602002002200141017236020420002005410372360204200041086a" +
            "0f0b20000f0b200141086a0ba203010b7f418882c0002d00001a41221001220704" +
            "402007410020076b41037122026a21062002044020072103418080c00021050340" +
            "200320052d00003a0000200541016a2105200341016a22032006490d000b0b2006" +
            "412220026b220b417c71220a6a210302402002418080406b22024103710440200a" +
            "41004c0d0120024103742208411871210c2002417c71220541046a210441002008" +
            "6b411871210820052802002105034020062005200c762004280200220520087472" +
            "360200200441046a2104200641046a22062003490d000b0c010b200a41004c0d00" +
            "20022104034020062004280200360200200441046a2104200641046a2206200349" +
            "0d000b0b2002200a6a2104200b41037122020440200220036a2102034020032004" +
            "2d00003a0000200441016a2104200341016a22032002490d000b0b200104402001" +
            "4122460440200021032007210241002105412221080240034020032d0000220920" +
            "022d00002204460440200341016a2103200241016a2102200841016b22080d010c" +
            "020b0b200920046b21050b20054521090b2000200110030b20074122100320090f" +
            "0b000bd20601047f0240200041046b280200220241787122034104410820024103" +
            "7122021b20016a4f0440200241002003200141276a4b1b0d01200041086b220120" +
            "0041046b280200220341787122006a21020240024020034101710d002003410271" +
            "450d012001280200220320006a2100200120036b220141bc85c000280200460440" +
            "20022802044103714103470d0141b485c000200036020020022002280204417e71" +
            "36020420012000410172360204200220003602000c020b2001200310080b024002" +
            "4002400240024020022802042203410271450440200241c085c000280200460d02" +
            "200241bc85c000280200460d0320022003417871220210082001200020026a2200" +
            "410172360204200020016a2000360200200141bc85c000280200470d0141b485c0" +
            "0020003602000c060b20022003417e713602042001200041017236020420002001" +
            "6a20003602000b2000418002490d022001200010074100210141d485c00041d485" +
            "c00028020041016b220036020020000d04419c83c0002802002200044003402001" +
            "41016a2101200028020822000d000b0b41d485c000200141ff1f200141ff1f4b1b" +
            "3602000c040b41c085c000200136020041b885c00041b885c00028020020006a22" +
            "003602002001200041017236020441bc85c000280200200146044041b485c00041" +
            "0036020041bc85c00041003602000b200041cc85c00028020022034d0d0341c085" +
            "c0002802002202450d034100210041b885c00028020022044129490d02419483c0" +
            "00210103402002200128020022054f04402002200520012802046a490d040b2001" +
            "28020821010c000b000b41bc85c000200136020041b485c00041b485c000280200" +
            "20006a220036020020012000410172360204200020016a20003602000c020b2000" +
            "41f8017141a483c0006a2102027f41ac85c0002802002203410120004103767422" +
            "007145044041ac85c000200020037236020020020c010b20022802080b21002002" +
            "20013602082000200136020c2001200236020c200120003602080c010b419c83c0" +
            "00280200220104400340200041016a2100200128020822010d000b0b41d485c000" +
            "200041ff1f200041ff1f4b1b360200200320044f0d0041cc85c000417f3602000b" +
            "0f0b418981c00041b881c0001004000b41c881c00041f881c0001004000b410101" +
            "7f230041206b220224002002410036021020024101360204200242043702082002" +
            "412e36021c200220003602182002200241186a360200200220011005000ba40201" +
            "037f230041206b22022400200241106a2203200041106a29020037030020024108" +
            "6a2204200041086a290200370300200241013b011c200220013602182002200029" +
            "0200370300230041206b2200240020022802182101200041106a20032902003703" +
            "00200041086a20042902003703002000200236021c200020013602182000200229" +
            "020037030041002102230041106b22012400200028020c21030240024002400240" +
            "20002802040e020001020b20030d01410121030c020b20030d0020002802002203" +
            "2802042102200328020021030c010b20014180808080783602002001200036020c" +
            "20014101200028021c22002d001c20002d001d1009000b20012002360204200120" +
            "0336020020014102200028021c22002d001c20002d001d1009000b090020004100" +
            "3602000bba0201047f411f210220004200370210200141ffffff074d0440200141" +
            "0620014108766722036b7641017120034101746b413e6a21020b2000200236021c" +
            "2002410274419482c0006a21044101200274220341b085c0002802007145044020" +
            "042000360200200020043602182000200036020c2000200036020841b085c00041" +
            "b085c0002802002003723602000f0b024002402001200428020022032802044178" +
            "71460440200321020c010b20014100411920024101766b2002411f461b74210503" +
            "4020032005411d764104716a41106a22042802002202450d022005410174210520" +
            "02210320022802044178712001470d000b0b20022802082201200036020c200220" +
            "00360208200041003602182000200236020c200020013602080f0b200420003602" +
            "00200020033602182000200036020c200020003602080bf10201047f200028020c" +
            "21020240024020014180024f044020002802182103024002402000200246044020" +
            "0041144110200028021422021b6a28020022010d01410021020c020b2000280208" +
            "2201200236020c200220013602080c010b200041146a200041106a20021b210403" +
            "40200421052001220241146a200241106a200228021422011b2104200241144110" +
            "20011b6a28020022010d000b200541003602000b2003450d022000200028021c41" +
            "0274419482c0006a220128020047044020034110411420032802102000461b6a20" +
            "023602002002450d030c020b2001200236020020020d0141b085c00041b085c000" +
            "280200417e200028021c77713602000c020b200028020822002002470440200020" +
            "0236020c200220003602080f0b41ac85c00041ac85c000280200417e2001410376" +
            "77713602000f0b2002200336021820002802102201044020022001360210200120" +
            "023602180b20002802142200450d0020022000360214200020023602180b0b7b01" +
            "017f230041106b22032400419082c000419082c000280200220441016a36020002" +
            "4020044100480d00024041dc85c0002d000045044041d885c00041d885c0002802" +
            "0041016a360200418c82c00028020041004e0d010c020b200341086a2000200111" +
            "0000000b41dc85c00041003a00002002450d00000b000b0c002000200129020037" +
            "03000b0b8f020100418080c0000b850272486239434a4157794234726a39315652" +
            "576e3936446b756b4734627764747954686361706163697479206f766572666c6f" +
            "77002200100011000000616c6c6f632f7372632f7261775f7665632e72733c0010" +
            "001400000018000000050000002f727573742f646570732f646c6d616c6c6f632d" +
            "302e322e362f7372632f646c6d616c6c6f632e7273617373657274696f6e206661" +
            "696c65643a207073697a65203e3d2073697a65202b206d696e5f6f766572686561" +
            "64006000100029000000a804000009000000617373657274696f6e206661696c65" +
            "643a207073697a65203c3d2073697a65202b206d61785f6f766572686561640000" +
            "6000100029000000ae0400000d00550970726f64756365727302086c616e677561" +
            "6765010452757374000c70726f6365737365642d62790105727573746325312e38" +
            "332e302d6e696768746c79202863326637346333663920323032342d30392d3039" +
            "2900490f7461726765745f6665617475726573042b0f6d757461626c652d676c6f" +
            "62616c732b087369676e2d6578742b0f7265666572656e63652d74797065732b0a" +
            "6d756c746976616c7565"

  console.log(finish, finish.length)

  const response2 = await submit({
    TransactionType: 'EscrowCreate',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Amount: "100000",
    Destination: wallet.address,
    CancelAfter: close_time + 20,
    FinishAfter: close_time + 2,
    FinishFunction: finish,
  }, seed)
  // console.log(JSON.stringify(response2.result, null, 4))
  if (response2.result.meta.TransactionResult !== "tesSUCCESS") process.exit(1);
  const sequence = response2.result.Sequence

  await sleep(5000)
  await client.request({command: 'ledger_accept'})

  const response3 = await submit({
    TransactionType: 'EscrowFinish',
    Account: wallet.address,
    Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    OfferSequence: sequence,
  }, wallet.seed)
  // console.log(JSON.stringify(response3.result, null, 4))

  const response4 = await submit({
    TransactionType: 'EscrowFinish',
    Account: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    Owner: "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
    OfferSequence: sequence,
  }, seed)

  await client.disconnect()
}

test()
