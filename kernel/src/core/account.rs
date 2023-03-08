use crate::core::nonce::Nonce;

use super::public_key_hash::PublicKeyHash;

pub struct Account {
    pub public_key_hash: PublicKeyHash,
    pub nonce: Nonce,
}

impl Account {
    /// Returns the nonce of the account
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }

    /// Returns the same account with an increment account
    pub fn increment_nonce(self) -> Account {
        Account {
            nonce: self.nonce.next(),
            ..self
        }
    }
}
