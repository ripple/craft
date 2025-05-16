use crate::hashing::HASH256_LEN;
use hex;
use xrpl::core::addresscodec::utils::decode_base58;

pub const ACCOUNT_ID_LEN: usize = 20;
pub type AccountId = Vec<u8>; //TODO size

pub enum Decodable {
    UINT256 = 5,
    AMOUNT = 6,
    ACCOUNT = 8,
    NOT,
}

impl Decodable {
    pub fn from_sfield(field: i32) -> Self {
        let field_type = field >> 16;
        match field_type {
            5 => Decodable::UINT256,
            6 => Decodable::AMOUNT,
            8 => Decodable::ACCOUNT,
            _ => Decodable::NOT,
        }
    }
}

pub fn decode_account_id(base58_account_id: &String) -> Option<Vec<u8>> {
    match decode_base58(base58_account_id, &[0x0]) {
        Ok(aid) => {
            if aid.len() == ACCOUNT_ID_LEN {
                Some(aid)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn decode_hash(hex_hash: &String) -> Option<Vec<u8>> {
    match hex::decode(hex_hash) {
        Ok(bytes) => {
            if bytes.len() == HASH256_LEN {
                Some(bytes)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub fn decode_amount(amount: &String) -> Option<Vec<u8>> {
    match amount.parse::<i64>() {
        Ok(num) => Some(num.to_le_bytes().to_vec()),
        Err(_) => None,
    }
}

pub fn decode(s: &String, decodable: Decodable) -> Option<Vec<u8>> {
    match decodable {
        Decodable::UINT256 => decode_hash(s),
        Decodable::ACCOUNT => decode_account_id(s),
        Decodable::AMOUNT => decode_amount(s),
        Decodable::NOT => Some(s.as_bytes().to_vec()),
    }
}
