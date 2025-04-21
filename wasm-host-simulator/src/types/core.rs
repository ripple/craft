use std::fmt::{Formatter, UpperHex};

// Represents the context retrieved by HOOK_SETUP
#[derive(Debug, Clone)]
pub(crate) struct ApplyContext {
    // Example: Assuming 'tx' is a field holding transaction info
    pub tx: Transaction,
    // ... other fields ...
}

#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    // Example methods mimicking the C++ code
    // In a real scenario, these would interact with your blockchain state
}

impl Transaction {
    pub(crate) fn get_transaction_id(&self) -> Hash256 {
        Hash256::dummy_tx_id()
    }
}

// Represents a 256-bit hash (like transaction ID)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Hash256([u8; 32]);

impl Hash256 {
    pub(crate) fn size(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn data(&self) -> &[u8] {
        &self.0
    }
    pub(crate) fn dummy_tx_id() -> Self {
        Hash256([0xFF; 32])
    }
}

impl UpperHex for Hash256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hex_string_upper = hex::encode_upper(self.0);
        write!(f, "{}", &hex_string_upper)?;
        Ok(())
    }
}
