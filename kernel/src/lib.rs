use crate::core::public_key_hash::PublicKeyHash;

// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;
use storage::{read_account, store_account};

mod constants;
mod core;
mod stages;
mod storage;

use crate::core::error::*;
use stages::{read_input, verify_nonce, verify_signature};

/// A step is processing only one message from the inbox
///
/// It will execute several sub steps:
/// - read the next message from the inbox
/// - verify its signature
/// - interpret the message
/// - save the result to the durable state
fn step<Host: RawRollupCore>(host: &mut Host) -> Result<()> {
    host.write_debug("Processing message\n");
    let message = read_input(host)?;
    let public_key = message.public_key();
    let public_key_hash = PublicKeyHash::from(public_key);
    host.write_debug("Message is deserialized\n");

    let inner = verify_signature(message)?;
    host.write_debug("Signature is correct\n");

    // Verify the nonce
    let account = read_account(host, &public_key_hash)?;
    let _content = verify_nonce(inner, account.nonce())?;
    let account = account.increment_nonce();
    let _ = store_account(host, &public_key_hash, &account)?;

    Ok(())
}

/// Process all the inbox
///
/// It also has the responsability to reboot the kernel and count ticks
fn execute<Host: RawRollupCore>(host: &mut Host) -> Result<()> {
    let result = step(host);
    match result {
        Ok(()) => Ok(()),
        Err(Error::SerdeJson(_)) => execute(host),
        Err(Error::FromUtf8Error(_)) => execute(host),
        Err(Error::EndOfInbox) => Ok(()),
        Err(Error::NotATzwitterMessage) => execute(host),
        Err(Error::Runtime(err)) => Err(Error::Runtime(err)),
        Err(Error::Ed25519Compact(_)) => execute(host),
        Err(Error::InvalidSignature) => execute(host),
        Err(Error::InvalidNonce) => execute(host),
        Err(Error::PathError(_)) => execute(host),
        Err(Error::StateDeserializarion) => execute(host),
    }
}

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");

    match execute(host) {
        Ok(()) => {}
        Err(err) => host.write_debug(&err.to_string()),
    }
}

kernel_entry!(entry);

#[cfg(test)]
mod tests {
    use mock_runtime::{host::MockHost, state::HostState};

    use crate::{constants::MAGIC_BYTE, step};

    fn valid_input() -> Vec<u8> {
        let input = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a226564736967746658484337537875433378754453423563624a426a786b514672656f6e38584368526750446f674547355662506542545250794341513156586a75734e4a375537456557674d44703679634159473334774851665667726d47454a6974227d2c22696e6e6572223a7b226e6f6e6365223a312c22636f6e74656e74223a7b22506f73745477656574223a7b22617574686f72223a7b22547a31223a22747a315146443957714c575a6d6d4175716e6e545050556a666175697459455764736876227d2c22636f6e74656e74223a2248656c6c6f20776f726c64227d7d7d7d";
        let msg = format!("01{:02x}{}", MAGIC_BYTE, input);

        hex::decode(msg).unwrap()
    }

    #[test]
    fn test_step() {
        let state = HostState::default();
        let input = valid_input();
        let inputs = [input.as_slice()].into_iter();
        let mut host = MockHost::from(state);

        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let res = step(&mut host);

        assert!(res.is_ok());
    }

    #[test]
    fn test_replay_attack() {
        let state = HostState::default();
        let input = valid_input();
        let inputs = [input.as_slice(), input.as_slice()].into_iter();
        let mut host = MockHost::from(state);
        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let res1 = step(&mut host);
        let res2 = step(&mut host);

        assert!(res1.is_ok());
        assert!(res2.is_err());
    }
}
