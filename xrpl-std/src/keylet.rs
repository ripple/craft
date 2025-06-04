use crate::host;
use crate::types::{AccountID, Keylet, XRPL_KEYLET_SIZE};

pub fn account_keylet(aid: &AccountID) -> Option<Keylet> {
    let mut key_let: Keylet = [0; XRPL_KEYLET_SIZE];
    let retcode = unsafe {
        host::account_keylet(aid.as_ptr(), aid.len(), key_let.as_mut_ptr(), key_let.len())
    };
    if retcode > 0 { Some(key_let) } else { None }
}

pub fn credential_keylet(
    subject: &AccountID,
    issuer: &AccountID,
    credential_type: &[u8],
) -> Option<Keylet> {
    let mut key_let: Keylet = [0; XRPL_KEYLET_SIZE];
    let retcode = unsafe {
        host::credential_keylet(
            subject.as_ptr(),
            subject.len(),
            issuer.as_ptr(),
            issuer.len(),
            credential_type.as_ptr(),
            credential_type.len(),
            key_let.as_mut_ptr(),
            key_let.len(),
        )
    };
    if retcode > 0 { Some(key_let) } else { None }
}

pub fn oracle_keylet(owner: &AccountID, document_id: i32) -> Option<Keylet> {
    let mut key_let: Keylet = [0; XRPL_KEYLET_SIZE];
    let retcode = unsafe {
        host::oracle_keylet(
            owner.as_ptr(),
            owner.len(),
            document_id,
            key_let.as_mut_ptr(),
            key_let.len(),
        )
    };
    if retcode > 0 { Some(key_let) } else { None }
}
