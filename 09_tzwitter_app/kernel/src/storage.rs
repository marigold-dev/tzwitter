use crate::core::public_key_hash::PublicKeyHash;
use crate::core::{account::Account, error::*, nonce::Nonce};
use host::runtime::load_value_slice;
use host::{
    path::{concat, OwnedPath, RefPath},
    rollup_core::RawRollupCore,
    runtime::Runtime,
};

const ACCOUNTS: RefPath = RefPath::assert_from(b"/accounts");

/// Compute the path /accounts/{tz1...}
fn account_path(public_key_hash: &PublicKeyHash) -> Result<OwnedPath> {
    let public_key_hash = public_key_hash.to_b58();
    let path: Vec<u8> = format!("/{}", public_key_hash).into();
    let path = OwnedPath::try_from(path).map_err(Error::from)?;
    concat(&ACCOUNTS, &path).map_err(Error::from)
}

/// Compute the path /accounts/{tz1...}/nonce
fn nonce_path(public_key_hash: &PublicKeyHash) -> Result<OwnedPath> {
    let nonce_path: Vec<u8> = "/nonce".into();
    let nonce_path = OwnedPath::try_from(nonce_path).map_err(Error::from)?;
    let account_path = account_path(public_key_hash)?;
    concat(&account_path, &nonce_path).map_err(Error::from)
}

/// Read the nonce of a given account
///
/// Returns the default value of the Nonce if it does not exists
fn read_nonce<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
) -> Result<Nonce> {
    let path = nonce_path(public_key_hash)?;
    let is_present = Runtime::store_has(host, &path)?
        .map(|_| true)
        .unwrap_or_default();

    match is_present {
        false => Ok(Nonce::default()),
        true => {
            let mut buffer = [0_u8; 8];
            match load_value_slice(host, &path, &mut buffer) {
                Ok(8) => {
                    let nonce = u64::from_be_bytes(buffer);
                    Ok(Nonce(nonce))
                }
                _ => Err(Error::StateDeserializarion),
            }
        }
    }
}

/// Store the nonce of an account
///
/// Deletes the old value
fn store_nonce<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    nonce: &'a Nonce,
) -> Result<&'a Nonce> {
    let path = nonce_path(public_key_hash)?;

    let data = nonce.0.to_be_bytes();
    let data = data.as_slice();

    host.store_write(&path, data, 0)
        .map_err(Error::from)
        .map(|_| nonce)
}

/// Read the account of the user
pub fn read_account<Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
) -> Result<Account> {
    let nonce = read_nonce(host, public_key_hash)?;
    Ok(Account { nonce })
}

pub fn store_account<'a, Host: RawRollupCore + Runtime>(
    host: &mut Host,
    public_key_hash: &PublicKeyHash,
    account: &'a Account,
) -> Result<&'a Account> {
    let Account { nonce } = account;
    let _ = store_nonce(host, public_key_hash, nonce)?;
    Ok(account)
}
