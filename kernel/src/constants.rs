pub const MAGIC_BYTE: u8 = 0x74;

#[cfg(not(debug_assertions))]
pub const L1_TOKEN_CONTRACT_ADDRESS: &'static str = env!("TZWITTER_L1_CONTRACT");
#[cfg(debug_assertions)]
pub const L1_TOKEN_CONTRACT_ADDRESS: &'static str = "KT1RycYvM4EVs6BAXWEsGXaAaRqiMP53KT4w";

pub const L1_TOKEN_CONTRACT_ENTRYPOINT: &'static str = "mint";
