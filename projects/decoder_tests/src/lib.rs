#![no_std]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use xrpl_std::decode_hex_32;
use xrpl_std::host::trace::{trace, trace_data, trace_num, DataRepr};
use xrpl_std::host::{
    cache_ledger_obj, get_ledger_obj_array_len, get_ledger_obj_field, get_ledger_obj_nested_field,
};
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield;
use xrpl_std::sfield::{
    Account, AccountTxnID, Balance, Domain, EmailHash, Flags, LedgerEntryType, MessageKey,
    OwnerCount, PreviousTxnID, PreviousTxnLgrSeq, RegularKey, Sequence, TicketCount, TransferRate,
};

fn test_account_root() {
    let _ = trace("\n$$$ test_account_root $$$");
    let keylet =
        decode_hex_32(b"13F1A95D7AAB7108D5CE7EEAF504B2894B8C674E6D68499076441C4837282BF8").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut out_buf = [0u8; 20];
    let out_len = unsafe {
        get_ledger_obj_field(slot, Account, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  Account:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = [0u8; 32];
    let out_len = unsafe {
        get_ledger_obj_field(slot, AccountTxnID, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  AccountTxnID:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf =[0u8; 48];
    let out_len = unsafe {
        get_ledger_obj_field(slot, Balance, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  Balance:", &out_buf[0..out_len], DataRepr::AsHex);

    // let _ = trace_num("  Balance:", out_buf as i64);

    let mut out_buf = [0u8; 20];
    let out_len =
        unsafe { get_ledger_obj_field(slot, Domain, out_buf.as_mut_ptr(), out_buf.len()) as usize };
    let _ = trace_data("  Domain:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = [0u8; 16];
    let out_len = unsafe {
        get_ledger_obj_field(slot, EmailHash, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  EmailHash:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, Flags, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  Flags:", out_buf as i64);

    let mut out_buf = 0i16;
    let out_len = unsafe {
        get_ledger_obj_field(
            slot,
            LedgerEntryType,
            (&mut out_buf) as *mut i16 as *mut u8,
            2,
        ) as usize
    };
    let _ = trace_num("  LedgerEntryType:", out_buf as i64);

    let mut out_buf = [0u8; 32];
    let out_len = unsafe {
        get_ledger_obj_field(slot, MessageKey, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  MessageKey:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, OwnerCount, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  OwnerCount:", out_buf as i64);

    let mut out_buf = [0u8; 32];
    let out_len = unsafe {
        get_ledger_obj_field(slot, PreviousTxnID, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  PreviousTxnID:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(
            slot,
            PreviousTxnLgrSeq,
            (&mut out_buf) as *mut i32 as *mut u8,
            4,
        ) as usize
    };
    let _ = trace_num("  PreviousTxnLgrSeq:", out_buf as i64);

    let mut out_buf = [0u8; 20];
    let out_len = unsafe {
        get_ledger_obj_field(slot, RegularKey, out_buf.as_mut_ptr(), out_buf.len()) as usize
    };
    let _ = trace_data("  RegularKey:", &out_buf[0..out_len], DataRepr::AsHex);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, Sequence, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  Sequence:", out_buf as i64);

    let mut out_buf = 0i32;
    let out_len = unsafe {
        get_ledger_obj_field(slot, TicketCount, (&mut out_buf) as *mut i32 as *mut u8, 4) as usize
    };
    let _ = trace_num("  TicketCount:", out_buf as i64);

    let mut out_buf = 0i64;
    let out_len = unsafe {
        get_ledger_obj_field(slot, TransferRate, (&mut out_buf) as *mut i64 as *mut u8, 4) as usize
    };
    let _ = trace_num("  TransferRate:", out_buf);

    //TODO urlgravatar is not an sfield, double check
}

fn test_amendments() {
    let _ = trace("\n$$$ test_amendments $$$");
    let keylet =
        decode_hex_32(b"7DB0788C020F02780A673DC74757F23823FA3014C1866E72CC4CD8B226CD6EF4").unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let array_len = unsafe { get_ledger_obj_array_len(slot, sfield::Amendments) };
    let _ = trace_num("  Amendments array len:", array_len as i64);
    for i in 0..array_len {
        let mut buf = [0x00; 32];
        let mut locator = LocatorPacker::new();
        locator.pack(sfield::Amendments);
        locator.pack(i);
        let output_len = unsafe {
            get_ledger_obj_nested_field(
                slot,
                locator.get_addr(),
                locator.num_packed_bytes(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        };
        let _ = trace_data("  Amendment:", &buf[..output_len as usize], DataRepr::AsHex);
    }

    let mut out_buf = 0i16;
    let out_len = unsafe {
        get_ledger_obj_field(
            slot,
            LedgerEntryType,
            (&mut out_buf) as *mut i16 as *mut u8,
            2,
        ) as usize
    };
    let _ = trace_num("  LedgerEntryType:", out_buf as i64);

    let mut buf = [0x00; 32];
    let mut locator = LocatorPacker::new();
    locator.pack(sfield::Majorities);
    locator.pack(0);
    locator.pack(sfield::Majority);
    locator.pack(sfield::Amendment);
    let output_len = unsafe {
        get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    let _ = trace_data(
        "  Majority Amendment:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );

    locator.repack_last(sfield::CloseTime);
    let mut out_buf = 0i64;
    let out_len = unsafe {
        get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            (&mut out_buf) as *mut i64 as *mut u8,
            4,
        ) as usize
    };
    let _ = trace_num("  Majority CloseTime:", out_buf);
}

// fn test_amm() {
//     let _ = trace("\n$$$ test_amm $$$");
// 
//     let keylet = <[u8; 32]>::try_from(
//         decode_hex_32(b"97DD92D4F3A791254A530BA769F6669DEBF6B2FC8CCA46842B9031ADCD4D1ADA").unwrap(),
//     )
//     .unwrap();
// 
//     let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };
// 
//     let mut buf = [0x00; 20];
//     let mut locator = LocatorPacker::new();
//     locator.pack(sfield::Asset);
//     locator.pack(sfield::IssueCurrency);
//     let output_len = unsafe {
//         get_ledger_obj_nested_field(
//             slot,
//             locator.get_addr(),
//             locator.num_packed_bytes(),
//             buf.as_mut_ptr(),
//             buf.len(),
//         )
//     };
//     let _ = trace_data(
//         "  Asset currency:",
//         &buf[..output_len as usize],
//         DataRepr::AsUTF8,
//     );
// 
//     let mut locator = LocatorPacker::new();
//     locator.pack(sfield::Asset2);
//     locator.pack(sfield::IssueCurrency);
//     let output_len = unsafe {
//         get_ledger_obj_nested_field(
//             slot,
//             locator.get_addr(),
//             locator.num_packed_bytes(),
//             buf.as_mut_ptr(),
//             buf.len(),
//         )
//     };
//     let _ = trace_data(
//         "  Asset2 currency:",
//         &buf[..output_len as usize],
//         DataRepr::AsUTF8,
//     );
// 
//     locator.repack_last(sfield::IssueIssuer);
//     let output_len = unsafe {
//         get_ledger_obj_nested_field(
//             slot,
//             locator.get_addr(),
//             locator.num_packed_bytes(),
//             buf.as_mut_ptr(),
//             buf.len(),
//         )
//     };
//     let _ = trace_data(
//         "  Asset2 issuer:",
//         &buf[..output_len as usize],
//         DataRepr::AsHex,
//     );
// 
//     let mut locator = LocatorPacker::new();
//     locator.pack(sfield::AuctionSlot);
//     locator.pack(sfield::Price);
//     locator.pack(sfield::IssueCurrency);
//     let output_len = unsafe {
//         get_ledger_obj_nested_field(
//             slot,
//             locator.get_addr(),
//             locator.num_packed_bytes(),
//             buf.as_mut_ptr(),
//             buf.len(),
//         )
//     };
//     let _ = trace_data(
//         "  AuctionSlot Price currency:",
//         &buf[..output_len as usize],
//         DataRepr::AsHex,
//     );
// 
//     locator.repack_last(sfield::IssueIssuer);
//     let output_len = unsafe {
//         get_ledger_obj_nested_field(
//             slot,
//             locator.get_addr(),
//             locator.num_packed_bytes(),
//             buf.as_mut_ptr(),
//             buf.len(),
//         )
//     };
//     let _ = trace_data(
//         "  AuctionSlot Price issuer:",
//         &buf[..output_len as usize],
//         DataRepr::AsHex,
//     );
// 
//     let mut value = 0u64;
//     locator.repack_last(sfield::IOUValue);
//     let output_len = unsafe {
//         get_ledger_obj_nested_field(
//             slot,
//             locator.get_addr(),
//             locator.num_packed_bytes(),
//             (&mut value) as *mut u64 as *mut u8,
//             8,
//         )
//     };
//     let _ = trace_num("  AuctionSlot Price value:", value as i64);
// }

fn test_amm() {
    let _ = trace("\n$$$ test_amm $$$");

    let keylet = <[u8; 32]>::try_from(
        decode_hex_32(b"97DD92D4F3A791254A530BA769F6669DEBF6B2FC8CCA46842B9031ADCD4D1ADA").unwrap(),
    )
        .unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut buf = [0x00; 100];
    let output_len = unsafe {
        get_ledger_obj_field(
            slot,
            sfield::LPTokenBalance,
            buf.as_mut_ptr(),
            buf.len(),
        )
    };

    let _ = trace_data("  get LPTokenBalance output_len:", &buf[..output_len as usize], DataRepr::AsHex,);
}

fn test_mpt() {
    let _ = trace("\n$$$ test_mpt, access individual fields $$$");

    let keylet = <[u8; 32]>::try_from(
        decode_hex_32(b"22F99DCD55BCCF3D68DC3E4D6CF12602006A7563A6BE93FC57FD63298BCCEB13").unwrap(),
    )
    .unwrap();

    let slot = unsafe { cache_ledger_obj(keylet.as_ptr(), keylet.len(), 0) };

    let mut buf = [0x00; 24];
    let output_len = unsafe {
        get_ledger_obj_field(slot, sfield::MPTokenIssuanceID, buf.as_mut_ptr(), buf.len())
    };
    let _ = trace_data(
        "  MPTokenIssuanceID:",
        &buf[..output_len as usize],
        DataRepr::AsHex,
    );

    let mut value = 0u64;
    let output_len = unsafe {
        get_ledger_obj_field(
            slot,
            sfield::MPTAmount,
            (&mut value) as *mut u64 as *mut u8,
            8,
        )
    };
    let _ = trace_num("  MPTAmount:", value as i64);
}

fn test_mpt_amount() {
    let _ = trace("\n$$$ test_mpt as amount $$$");
}

#[no_mangle]
pub extern "C" fn finish() -> i32 {
    test_account_root();
    // test_amendments();
    test_amm();
    // test_mpt();
    // test_mpt_amount();
    //test_directory_node();
    //test_offer();
    //test_ripple_state();
    //test_check();
    //test_pay_channel();
    //test_deposit_preauth();
    //test_ticket();
    //test_nft_token_page();
    1
}
