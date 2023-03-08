use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct Nonce(pub u64);

impl Nonce {
    pub fn next(&self) -> Nonce {
        Nonce(self.0 + 1)
    }
}

impl ToString for Nonce {
    fn to_string(&self) -> String {
        format!("{:08X}", self.0)
    }
}
