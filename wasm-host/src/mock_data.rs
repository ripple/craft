#![allow(dead_code)]

use crate::decoding::{decode, AccountId, Decodable, SField_To_Name};
use crate::hashing::Hash256;
use std::collections::HashMap;

pub type Bytes = Vec<u8>;

pub type Keylet = Hash256;

pub enum DataSource {
    Tx,
    CurrentLedgerObj,
    KeyletLedgerObj(Keylet),
}

#[derive(Debug)]
pub struct MockData {
    tx: serde_json::Value,
    hosting_ledger_obj: serde_json::Value,
    header: serde_json::Value,
    ledger: HashMap<Keylet, serde_json::Value>,
    nfts: HashMap<Hash256, (AccountId, serde_json::Value)>,
}

impl MockData {
    pub fn new(
        tx_str: &String,
        hosting_ledger_obj_str: &String,
        header_str: &String,
        ledger_str: &String,
        nfts_str: &String,
    ) -> Self {
        let tx = serde_json::from_str(tx_str).expect("Tx JSON bad formatted");
        let hosting_ledger_obj = serde_json::from_str(hosting_ledger_obj_str)
            .expect("Hosting ledger object JSON bad formatted");
        let header = serde_json::from_str(header_str).expect("Ledger header JSON bad formatted");

        let ledger = {
            let parsed_data: Vec<HashMap<String, serde_json::Value>> =
                serde_json::from_str(ledger_str).expect("Ledger JSON bad formatted");
            let mut combined_hashmap: HashMap<Keylet, serde_json::Value> = HashMap::new();
            for map_entry in parsed_data {
                for (key, value) in map_entry {
                    let keylet: Keylet =
                        decode(&key, Decodable::UINT256).expect("ledger file, bad keylet");
                    // println!("MockData keylet {:?}", keylet);
                    combined_hashmap.insert(keylet, value);
                }
            }
            combined_hashmap
        };

        let nfts = {
            let mut nft_map = HashMap::new();
            let parsed_json: serde_json::Value =
                serde_json::from_str(nfts_str).expect("Failed to parse NFT JSON");
            for item in parsed_json.as_array().expect("NFT JSON not an array") {
                let nft_id = item["nft_id"].as_str();
                let owner = item["owner"].as_str();
                let uri = item.get("uri");

                if let (Some(id), Some(owner), Some(uri)) = (nft_id, owner, uri) {
                    nft_map.insert(
                        decode(&id.to_string(), Decodable::UINT256).expect("NFT file, bad nft_id"),
                        (
                            decode(&owner.to_string(), Decodable::ACCOUNT)
                                .expect("NFT file, bad owner"),
                            uri.clone(),
                        ),
                    );
                } else {
                    panic!("NFT missing field(s)");
                }
            }
            nft_map
        };

        MockData {
            tx,
            hosting_ledger_obj,
            header,
            ledger,
            nfts,
        }
    }

    pub fn obj_exist(&self, keylet: &Keylet) -> bool {
        self.ledger.get(keylet).is_some()
    }

    #[inline]
    fn get_field_name(&self, field_id: i32) -> Option<String> {
        SField_To_Name.get(&field_id).cloned()
    }

    pub fn get_field_value(
        &self,
        source: DataSource,
        idx_fields: Vec<i32>,
    ) -> Option<&serde_json::Value> {
        let mut curr = match source {
            DataSource::Tx => &self.tx,
            DataSource::CurrentLedgerObj => &self.hosting_ledger_obj,
            DataSource::KeyletLedgerObj(obj_hash) => self.ledger.get(&obj_hash)?,
        };

        for idx_field in idx_fields {
            if curr.is_array() {
                curr = curr.as_array().unwrap().get(idx_field as usize)?;
            } else {
                curr = curr.get(self.get_field_name(idx_field)?)?;
            }
        }
        Some(curr)
    }

    pub fn get_array_len(&self, source: DataSource, idx_fields: Vec<i32>) -> Option<usize> {
        let value = self.get_field_value(source, idx_fields)?;
        if value.is_array() {
            Some(value.as_array()?.len())
        } else {
            None
        }
    }

    pub fn get_ledger_sqn(&self) -> Option<&serde_json::Value> {
        self.header.get("ledger_index")
    }

    pub fn get_parent_ledger_time(&self) -> Option<&serde_json::Value> {
        self.header.get("parent_close_time")
    }

    pub fn get_parent_ledger_hash(&self) -> Option<&serde_json::Value> {
        self.header.get("parent_hash")
    }

    pub fn set_current_ledger_obj_data(&mut self, data: Vec<u8>) {
        self.hosting_ledger_obj["data"] = serde_json::Value::from(data);
    }

    pub fn get_nft_uri(
        &self,
        nft_id: &Hash256,
        account_id: &AccountId,
    ) -> Option<&serde_json::Value> {
        match self.nfts.get(nft_id) {
            None => None,
            Some((aid, uri)) => {
                if account_id == aid {
                    Some(uri)
                } else {
                    None
                }
            }
        }
    }
}
