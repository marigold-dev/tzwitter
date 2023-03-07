use crypto::hash::PublicKeyEd25519;

pub enum PublicKey {
    Ed25519(PublicKeyEd25519),
}

impl PublicKey {
    pub fn to_b58(&self) -> String {
        match self {
            PublicKey::Ed25519(pk) => pk.to_base58_check(),
        }
    }

    pub fn from_b58(data: &str) -> Result<Self, &'static str> {
        let ed25519 = PublicKeyEd25519::from_base58_check(data).ok();
        match ed25519 {
            Some(pkey) => Ok(PublicKey::Ed25519(pkey)),
            None => Err("Cannot decode b58"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PublicKey;

    #[test]
    fn test_ed25519_pk_deserialization() {
        let pkey = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK";
        let res = PublicKey::from_b58(pkey);
        assert!(res.is_ok());
    }

    #[test]
    fn test_ed25519_pk_serialization() {
        let pkey = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK";
        let serialized = PublicKey::from_b58(pkey).unwrap().to_b58();
        assert_eq!(pkey, &serialized)
    }
}
