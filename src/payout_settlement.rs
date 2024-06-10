use std::{collections::HashMap, fmt::Error};

use bitcoin::{secp256k1::SecretKey, Address, Transaction};

pub struct PayoutSettlement {
    transaction: Transaction,
}

impl PayoutSettlement {
    pub fn new(latest_eltoo_out_address: Address, payout_map: HashMap<Address, u64>) -> Self {
        // Implementation for building the payout settlement transaction
        unimplemented!();
    }

    pub fn add_sig(&mut self, private_key: &SecretKey) -> Result<(), Error> {
        // Implementation for adding the signature
        unimplemented!();
    }

    pub fn build(self) -> Transaction {
        self.transaction
    }
}
