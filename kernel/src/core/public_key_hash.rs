use crypto::hash::ContractTz1Hash;
use serde::{Deserialize, Serialize};

use crate::core::public_key::PublicKey;

use super::hash::Blake2b20;

#[derive(Deserialize, Serialize)]
pub enum PublicKeyHash {
    Tz1(ContractTz1Hash),
}

impl PublicKeyHash {
    pub fn from_b58(data: &str) -> Result<Self, &'static str> {
        let tz1 = ContractTz1Hash::from_base58_check(data).ok();
        match tz1 {
            Some(tz1) => Ok(PublicKeyHash::Tz1(tz1)),
            None => Err("Cannot parse public key hash"),
        }
    }

    pub fn to_b58(&self) -> String {
        match self {
            PublicKeyHash::Tz1(tz1) => tz1.to_base58_check(),
        }
    }
}

impl From<PublicKey> for PublicKeyHash {
    fn from(pkey: PublicKey) -> Self {
        match pkey {
            PublicKey::Ed25519(ed25519) => {
                let data = ed25519.as_ref();
                let hash = Blake2b20::from(data);

                let res = ContractTz1Hash::try_from(hash.as_ref());
                match res {
                    Ok(res) => PublicKeyHash::Tz1(res),
                    Err(_) => panic!(),
                }
            }
        }
    }
}

impl<'a> From<&'a PublicKey> for PublicKeyHash {
    fn from(pkey: &'a PublicKey) -> Self {
        match pkey {
            PublicKey::Ed25519(ed25519) => {
                let data = ed25519.as_ref();
                let hash = Blake2b20::from(data);
                let res = ContractTz1Hash::try_from(hash.as_ref());
                match res {
                    Ok(res) => PublicKeyHash::Tz1(res),
                    Err(_) => panic!(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::public_key::PublicKey;

    use super::PublicKeyHash;

    #[test]
    fn test_tz1_deserialization() {
        let tz1 = "tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv";
        let res = PublicKeyHash::from_b58(tz1);
        assert!(res.is_ok());
    }

    #[test]
    fn test_tz1_serializarion() {
        let tz1 = "tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv";
        let serialized = PublicKeyHash::from_b58(tz1).unwrap().to_b58();
        assert_eq!(tz1, &serialized);
    }

    #[test]
    fn test_tz1_from_pkey_serializarion() {
        let tz1 = "tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv";
        let pkey =
            PublicKey::from_b58("edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK").unwrap();

        let result = PublicKeyHash::from(pkey);

        assert_eq!(tz1, &result.to_b58())
    }
}
