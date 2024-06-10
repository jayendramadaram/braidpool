use std::fmt::Error;

use bitcoin::{secp256k1::SecretKey, Address, Transaction};

pub struct PayoutUpdate {
    transaction: Transaction,
}

impl PayoutUpdate {
    pub fn new(
        prev_update_tx: Option<Transaction>,
        coinbase_tx: Transaction,
        next_out_address: Address,
    ) -> Self {
        // Implementation for building the payout update transaction
        unimplemented!();
    }

    pub fn add_coinbase_sig(&mut self, private_key: &SecretKey) -> Result<(), Error> {
        // Implementation for adding the coinbase signature
        unimplemented!();
    }

    pub fn add_prevout_sig(&mut self, private_key: &SecretKey) -> Result<(), Error> {
        // Implementation for adding the previous output signature
        unimplemented!();
    }

    pub fn build(self) -> Transaction {
        self.transaction
    }
}
