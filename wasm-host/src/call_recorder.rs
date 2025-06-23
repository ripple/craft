use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostCall {
    pub function: String,
    pub call_order: usize,
    pub parameters: HostCallParams,
    pub return_value: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostCallParams {
    Trace {
        message: String,
        data: Option<Vec<u8>>,
        as_hex: u32,
    },
    UpdateData {
        data: Vec<u8>,
        data_len: usize,
    },
    GetTxField {
        field: i32,
        out_buf_cap: usize,
    },
    LedgerSlotSet {
        keylet: Vec<u8>,
        slot_num: i32,
    },
    GetLedgerObjField {
        slot: i32,
        field: i32,
        out_buf_cap: usize,
    },
}

#[derive(Debug, Default)]
pub struct CallRecorder {
    pub calls: VecDeque<HostCall>,
    pub call_counter: usize,
}

impl CallRecorder {
    pub fn new() -> Self {
        Self {
            calls: VecDeque::new(),
            call_counter: 0,
        }
    }

    pub fn record_trace(&mut self, message: String, data: Option<Vec<u8>>, as_hex: u32) -> i32 {
        self.call_counter += 1;
        let call = HostCall {
            function: "trace".to_string(),
            call_order: self.call_counter,
            parameters: HostCallParams::Trace {
                message: message.clone(),
                data: data.clone(),
                as_hex,
            },
            return_value: Some(0), // trace always returns success
        };
        self.calls.push_back(call);

        // Original trace behavior
        if let Some(data) = &data {
            if as_hex == 1 {
                println!(
                    "WASM TRACE: {} ({} | {} data bytes)",
                    message,
                    hex::encode(data),
                    data.len()
                );
            } else {
                println!(
                    "WASM TRACE: {} ({:?} | {} data bytes)",
                    message,
                    data,
                    data.len()
                );
            }
        } else {
            println!("WASM TRACE: {}", message);
        }

        0
    }

    pub fn record_update_data(&mut self, data: Vec<u8>) -> i32 {
        self.call_counter += 1;
        let call = HostCall {
            function: "update_data".to_string(),
            call_order: self.call_counter,
            parameters: HostCallParams::UpdateData {
                data: data.clone(),
                data_len: data.len(),
            },
            return_value: Some(0),
        };
        self.calls.push_back(call);

        println!(
            "WASM HOST: update_data called with {} bytes: {}",
            data.len(),
            hex::encode(&data)
        );
        0
    }

    #[allow(dead_code)]
    pub fn record_get_tx_field(&mut self, field: i32, out_buf_cap: usize) -> i32 {
        self.call_counter += 1;
        let call = HostCall {
            function: "get_tx_field".to_string(),
            call_order: self.call_counter,
            parameters: HostCallParams::GetTxField { field, out_buf_cap },
            return_value: Some(0),
        };
        self.calls.push_back(call);
        0
    }

    pub fn get_calls(&self) -> &VecDeque<HostCall> {
        &self.calls
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.calls.clear();
        self.call_counter = 0;
    }
}
