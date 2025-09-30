#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::locator::Locator;
use xrpl_wasm_std::decode_hex_32;
use xrpl_wasm_std::host::trace::{trace, trace_data, trace_num, DataRepr::AsHex};
use xrpl_wasm_std::host::{
    account_keylet, amm_keylet, cache_ledger_obj, check_keylet, escrow_keylet, get_ledger_obj_array_len,
    get_ledger_obj_field, get_ledger_obj_nested_array_len, get_ledger_obj_nested_field, get_ledger_sqn,
    line_keylet, mpt_issuance_keylet, mptoken_keylet, nft_offer_keylet, offer_keylet, paychan_keylet,
    permissioned_domain_keylet, signers_keylet, ticket_keylet, vault_keylet,
};
use xrpl_wasm_std::sfield;
use xrpl_address_macro::r_address;

const ACCOUNT: [u8; 20] = r_address!("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");

fn cache_and_check(slot: i32, field: i32, label: &str) -> i32 {
    let mut buf = [0u8; 64];
    let wrote = unsafe { get_ledger_obj_field(slot, field, buf.as_mut_ptr(), buf.len()) };
    if wrote <= 0 {
        let _ = trace_num("get_ledger_obj_field failed", wrote as i64);
        return wrote;
    }
    let _ = trace_data(label, &buf[..(wrote as usize).min(buf.len())], AsHex);
    0
}

fn compute_and_cache_keylet(label: &str, keylet_out: &mut [u8; 32]) -> i32 {
    let wrote = unsafe { account_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), keylet_out.as_mut_ptr(), keylet_out.len()) };
    if wrote <= 0 { return wrote; }
    let _ = trace_data(label, keylet_out, AsHex);
    unsafe { cache_ledger_obj(keylet_out.as_ptr(), keylet_out.len(), 0) }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let _ = trace("=== Keylets example ===");

    // General ledger info
    let ls = unsafe { get_ledger_sqn() };
    if ls <= 0 { return ls; }
    let _ = trace_num("ledger_sqn", ls as i64);

    // AccountRoot keylet -> cache -> read a few fields
    let mut kl = [0u8; 32];
    let slot = compute_and_cache_keylet("account_keylet", &mut kl);
    if slot <= 0 { return slot; }

    // Sanity check some fields on AccountRoot
    if cache_and_check(slot, sfield::Account, "Account") < 0 { return -1; }
    if cache_and_check(slot, sfield::Balance, "Balance") < 0 { return -1; }

    // Nested field example: no nesting on AccountRoot; demonstrate locator against array using fake path
    // Instead, show array len for Signers on SignerList (will likely be 0 in default fixtures)
    // Derive SignerList keylet and test array length + nested field access patterns
    let wrote = unsafe { signers_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), kl.as_mut_ptr(), kl.len()) };
    if wrote <= 0 { return wrote; }
    let _ = trace_data("signers_keylet", &kl, AsHex);
    let signer_slot = unsafe { cache_ledger_obj(kl.as_ptr(), kl.len(), 0) };
    if signer_slot > 0 {
        let count = unsafe { get_ledger_obj_array_len(signer_slot, sfield::Signers) };
        let _ = trace_num("Signer count", count as i64);
        if count > 0 {
            let mut loc = Locator::new();
            loc.pack(sfield::Signers);
            loc.pack(0);
            loc.pack(sfield::Account);
            let mut out = [0u8; 32];
            let wrote = unsafe {
                get_ledger_obj_nested_field(
                    signer_slot,
                    loc.get_addr(),
                    loc.num_packed_bytes(),
                    out.as_mut_ptr(),
                    out.len(),
                )
            };
            if wrote > 0 {
                let _ = trace_data("First signer Account", &out[..(wrote as usize)], AsHex);
            }
        }
    }

    // Showcase additional keylets (trace only):
    let mut out = [0u8; 32];
    let wrote = unsafe { escrow_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 1000, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("escrow_keylet", &out, AsHex); }
    let wrote = unsafe { check_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 1001, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("check_keylet", &out, AsHex); }
    let wrote = unsafe { ticket_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 1002, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("ticket_keylet", &out, AsHex); }
    let wrote = unsafe { offer_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 1003, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("offer_keylet", &out, AsHex); }
    let wrote = unsafe { nft_offer_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 1004, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("nft_offer_keylet", &out, AsHex); }
    let wrote = unsafe { paychan_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), r_address!("rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW").as_ptr(), 20, 1005, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("paychan_keylet", &out, AsHex); }
    let wrote = unsafe { permissioned_domain_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 1, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("permissioned_domain_keylet", &out, AsHex); }
    let wrote = unsafe { vault_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), 2, out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("vault_keylet", &out, AsHex); }

    // AMM and trust line-like keylets require issues and currency data; skip detailed cache check
    let issuer = ACCOUNT;
    let currency = [0u8; 20]; // placeholder 20 bytes; simulator expects 20-byte currency
    let wrote = unsafe { line_keylet(ACCOUNT.as_ptr(), ACCOUNT.len(), issuer.as_ptr(), issuer.len(), currency.as_ptr(), currency.len(), out.as_mut_ptr(), out.len()) };
    if wrote > 0 { let _ = trace_data("line_keylet", &out, AsHex); }

    // mpt issuance and token keylets – provide simple params
    let wrote = unsafe { mpt_issuance_keylet(issuer.as_ptr(), issuer.len(), 10, out.as_mut_ptr(), out.len()) };
    if wrote > 0 {
        let _ = trace_data("mpt_issuance_keylet", &out, AsHex);
        let mut holder = ACCOUNT;
        let wrote2 = unsafe { mptoken_keylet(out.as_ptr(), out.len(), holder.as_mut_ptr(), holder.len(), out.as_mut_ptr(), out.len()) };
        if wrote2 > 0 { let _ = trace_data("mptoken_keylet", &out, AsHex); }
    }

    1
}

