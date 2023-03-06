macro_rules! define_blake2b {
    ($name:ident, $size:expr) => {
        pub struct $name {
            inner: [u8; $size],
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                self.inner
                    .iter()
                    .fold("".to_string(), |acc, elt| format!("{}{:02x?}", acc, elt))
            }
        }

        impl<'a> From<&'a [u8]> for $name {
            fn from(data: &'a [u8]) -> Self {
                let digest = crypto::blake2b::digest(data, $size).unwrap();
                Self {
                    inner: digest.try_into().unwrap(),
                }
            }
        }

        impl<'a> From<&'a Vec<u8>> for $name {
            fn from(data: &'a Vec<u8>) -> Self {
                let data = data.as_slice();
                Self::from(data)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.inner
            }
        }
    };
}

define_blake2b!(Blake2b, 32);
define_blake2b!(Blake2b20, 20);

#[cfg(test)]
mod tests {
    use super::Blake2b;

    #[test]
    fn test_hash() {
        let data: &[u8] = [0x1, 0x2, 0x3, 0x4].as_slice();
        let hash = Blake2b::from(data);

        assert_eq!(
            "28517e4cdf6c90798c1a983b03727ca7743c21a3880672429ccfc5bd15ea5f72",
            hash.to_string()
        )
    }
}
