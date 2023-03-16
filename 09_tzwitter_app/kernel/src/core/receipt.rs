use crate::core::error::*;
use crate::core::hash::*;

/// definition of a receipt
///
/// For now we don't need to know the cause of the error
/// Because the receipt is only used in the front-end application to give user feedbacks
pub struct Receipt {
    hash: Blake2b,
    success: bool,
}

impl Receipt {
    pub fn new(hash: Blake2b, result: &Result<()>) -> Receipt {
        Receipt {
            hash,
            success: result.is_ok(),
        }
    }

    /// Returns the hash of the receipt
    pub fn hash(&self) -> &Blake2b {
        &self.hash
    }

    /// Returns a boolean that indicates if the receipt is a success or not
    pub fn success(&self) -> bool {
        self.success
    }
}
