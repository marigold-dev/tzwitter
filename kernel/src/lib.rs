use crate::core::message::{Content, Message};
use crate::core::public_key_hash::PublicKeyHash;
use crate::core::receipt::Receipt;

// src/lib.rs
use host::{rollup_core::RawRollupCore, runtime::Runtime};
use kernel::kernel_entry;
use storage::{read_account, store_account, store_receipt};

mod constants;
mod core;
mod stages;
mod storage;

use crate::core::error::*;
use stages::{
    create_tweet, get_previous_block_hash, like_tweet, read_input, transfer_tweet, verify_nonce,
    verify_signature, withdraw_tweet,
};

/// A step is processing only one message from the inbox
///
/// It will execute several sub steps:
/// - verify the signature of the message
/// - verify the nonce of the message
/// - handle the message
fn step<Host: RawRollupCore>(host: &mut Host, message: Message, previous_hash: &str) -> Result<()> {
    let public_key = message.public_key();
    let public_key_hash = PublicKeyHash::from(public_key);
    host.write_debug("Message is deserialized\n");

    let inner = verify_signature(message)?;
    host.write_debug("Signature is correct\n");

    // Verify the nonce
    let account = read_account(host, public_key_hash)?;
    let content = verify_nonce(inner, account.nonce())?;
    let account = account.increment_nonce();
    let _ = store_account(host, &account)?;

    // Interpret the message
    let () = match content {
        Content::PostTweet(post_tweet) => create_tweet(host, &account, post_tweet)?,
        Content::LikeTweet(tweet_id) => like_tweet(host, &account, &tweet_id)?,
        Content::Transfer(transfer) => transfer_tweet(host, &account, &transfer)?,
        Content::Collect(twwet_id) => withdraw_tweet(host, &previous_hash, &account, &twwet_id)?,
    };

    Ok(())
}

/// Process all the inbox
///
/// Read a message, process the error of the read message
/// If the message is correctly deserialized it continue the execution
/// Then all the errors, will be stored in a receipt
/// Continue until the inbox is emptied
///
/// This function stop its execution when a RuntimeError happens
///
/// TODO: it can count ticks and reboot the kernel between two inbox message
fn execute<Host: RawRollupCore>(host: &mut Host, previous_hash: &str) -> Result<()> {
    let message = read_input(host);
    match message {
        Err(ReadInputError::EndOfInbox) => Ok(()),
        Err(ReadInputError::Runtime(err)) => Err(Error::Runtime(err)),
        Err(_) => execute(host, previous_hash),
        Ok(message) => {
            // If the message is processed we can extract the hash of the message
            let hash = message.hash();
            let result = step(host, message, previous_hash);

            let receipt = Receipt::new(hash, &result);
            let _ = store_receipt(host, &receipt)?;

            match result {
                Err(Error::Runtime(err)) => Err(Error::Runtime(err)),
                Err(_) => execute(host, previous_hash),
                Ok(()) => execute(host, previous_hash),
            }
        }
    }
}

fn entry<Host: RawRollupCore>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");

    match get_previous_block_hash(host) {
        Err(err) => host.write_debug(&err.to_string()),
        Ok(previous_hash) => match execute(host, &previous_hash) {
            Ok(()) => {}
            Err(err) => host.write_debug(&err.to_string()),
        },
    }
}

kernel_entry!(entry);

#[cfg(test)]
mod tests {
    use host::{path::RefPath, rollup_core::RawRollupCore, runtime::Runtime};
    use mock_runtime::{host::MockHost, state::HostState};

    use crate::{
        constants::MAGIC_BYTE,
        core::message::Message,
        stages::read_input,
        step,
        storage::{exists, read_u64},
    };

    /// Assert a path exists in the storage
    fn assert_exist<Host: RawRollupCore + Runtime>(host: &mut Host, path: &str) {
        let path = RefPath::assert_from(path.as_bytes());
        let is_present = exists(host, &path).unwrap();
        assert!(is_present);
    }

    /// Assert a u64 value in the storage
    fn assert_u64<Host: RawRollupCore + Runtime>(
        host: &mut Host,
        path: &str,
        expected: Option<u64>,
    ) {
        let path = RefPath::assert_from(path.as_bytes());
        let value = read_u64(host, &path).unwrap();
        assert_eq!(expected, value);
    }

    fn assert_not_exists<Host: RawRollupCore + Runtime>(host: &mut Host, path: &str) {
        let path = RefPath::assert_from(path.as_bytes());
        let is_present = exists(host, &path).unwrap();
        assert!(!is_present)
    }

    /// Valid input that represent the content "Hello world" and the nonce 0
    fn input_1() -> Vec<u8> {
        let input = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a226564736967746658484337537875433378754453423563624a426a786b514672656f6e38584368526750446f674547355662506542545250794341513156586a75734e4a375537456557674d44703679634159473334774851665667726d47454a6974227d2c22696e6e6572223a7b226e6f6e6365223a312c22636f6e74656e74223a7b22506f73745477656574223a7b22617574686f72223a7b22547a31223a22747a315146443957714c575a6d6d4175716e6e545050556a666175697459455764736876227d2c22636f6e74656e74223a2248656c6c6f20776f726c64227d7d7d7d";
        let msg = format!("01{:02x}{}", MAGIC_BYTE, input);
        hex::decode(msg).unwrap()
    }

    /// Valid input that represent the content "Hello world" and the nonce 1
    fn input_2() -> Vec<u8> {
        let input = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a226564736967745a6647345a51346263746f65427a3166437053745141525473695154466974567067756652786d366b365a743478596e3432675647694d447634426236376331536d6f793270514b376569666533387148327455756f69627344597a6d227d2c22696e6e6572223a7b226e6f6e6365223a322c22636f6e74656e74223a7b22506f73745477656574223a7b22617574686f72223a7b22547a31223a22747a315146443957714c575a6d6d4175716e6e545050556a666175697459455764736876227d2c22636f6e74656e74223a2248656c6c6f20776f726c64227d7d7d7d";
        let msg = format!("01{:02x}{}", MAGIC_BYTE, input);
        hex::decode(msg).unwrap()
    }

    /// Create a like for tweet 0 with counter 1
    fn input_like() -> Vec<u8> {
        let input = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a226564736967746b717577626a4a467a41464c7134345267527454564e777948774857624b386e47343855564b5069766b32635057505735345359335935534e4439786635463852795335424e665861595a4c453664776d554b70325541394275435a32227d2c22696e6e6572223a7b226e6f6e6365223a322c22636f6e74656e74223a7b224c696b655477656574223a307d7d7d";
        let msg = format!("01{:02x}{}", MAGIC_BYTE, input);
        hex::decode(msg).unwrap()
    }

    fn input_like_2() -> Vec<u8> {
        let input = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a22656473696774775a6d6376566470575361696836646a5057526172645668723154614b32786275646a7937686d7a6a65456e4b77766747346d50676455573478764254714452584e5348596f6a5973395a796d5968565469586d667a67323778624846227d2c22696e6e6572223a7b226e6f6e6365223a332c22636f6e74656e74223a7b224c696b655477656574223a307d7d7d";
        let msg = format!("01{:02x}{}", MAGIC_BYTE, input);
        hex::decode(msg).unwrap()
    }

    fn input_transfer() -> Vec<u8> {
        let input = "7b22706b6579223a7b2245643235353139223a226564706b75444d556d375935337770346778654c425875694168585a724c6e385842315238336b737676657348384c7038626d43664b227d2c227369676e6174757265223a7b2245643235353139223a226564736967746a616a43534e5548464a6f6f775978756e566b5a53644478655a7459687a5756444d617359785365315a59625650444e4b4d4157574152454c52734244624242774d646f786f36676e36766639374e74413661745232637656746f7a37227d2c22696e6e6572223a7b226e6f6e6365223a322c22636f6e74656e74223a7b225472616e73666572223a7b2264657374696e6174696f6e223a7b22547a31223a22747a3154477536544e354753657a326e645858654458364c675544764c7a504c71675956227d2c2274776565745f6964223a307d7d7d7d";
        let msg = format!("01{:02x}{}", MAGIC_BYTE, input);
        hex::decode(msg).unwrap()
    }

    fn next_input<Host: RawRollupCore + Runtime>(host: &mut Host) -> Message {
        read_input(host).unwrap()
    }

    #[test]
    fn test_step() {
        let state = HostState::default();
        let input = input_1();
        let inputs = [input.as_slice()].into_iter();
        let mut host = MockHost::from(state);

        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let message = next_input(&mut host);
        let res = step(&mut host, message, "previous_hash");

        assert!(res.is_ok());

        assert_exist(&mut host, "/tweets/0");
        assert_u64(&mut host, "/tweets/0/likes", Some(0));
        assert_exist(
            &mut host,
            "/accounts/tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv/tweets/owned/0",
        );
        assert_exist(
            &mut host,
            "/accounts/tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv/tweets/written/0",
        );
    }

    #[test]
    fn test_replay_attack() {
        let state = HostState::default();
        let input = input_1();
        let inputs = [input.as_slice(), input.as_slice()].into_iter();
        let mut host = MockHost::from(state);
        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let message = next_input(&mut host);
        let res1 = step(&mut host, message, "previous_hash");
        let message = next_input(&mut host);
        let res2 = step(&mut host, message, "previous_hash");

        assert!(res1.is_ok());
        assert!(res2.is_err());
    }

    #[test]
    fn test_identical_tweets() {
        let state = HostState::default();
        let input_1 = input_1();
        let input_2 = input_2();
        let inputs = [input_1.as_slice(), input_2.as_slice()].into_iter();
        let mut host = MockHost::from(state);
        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let message = next_input(&mut host);
        let res_1 = step(&mut host, message, "previous_hash");
        let message = next_input(&mut host);
        let res_2 = step(&mut host, message, "previous_hash");

        assert!(res_1.is_ok());
        assert!(res_2.is_ok());

        assert_u64(&mut host, "/constants/tweet-counter", Some(2));
        assert_exist(&mut host, "/tweets/0");
        assert_exist(&mut host, "/tweets/1");
    }

    #[test]
    fn test_like() {
        let state = HostState::default();
        let input_1 = input_1();
        let input_2 = input_like();
        let inputs = [input_1.as_slice(), input_2.as_slice()].into_iter();
        let mut host = MockHost::from(state);

        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let message = next_input(&mut host);
        let res_1 = step(&mut host, message, "previous_hash");
        let message = next_input(&mut host);
        let res_2 = step(&mut host, message, "previous_hash");

        assert!(res_1.is_ok());
        assert!(res_2.is_ok());

        assert_u64(&mut host, "/tweets/0/likes", Some(1));
    }

    #[test]
    fn test_like_two_times_same_tweet() {
        let state = HostState::default();
        let input_1 = input_1();
        let input_2 = input_like();
        let input_3 = input_like_2();
        let inputs = [input_1.as_slice(), input_2.as_slice(), input_3.as_slice()].into_iter();

        let mut host = MockHost::from(state);
        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let message = next_input(&mut host);
        let res_1 = step(&mut host, message, "previous_hash");
        let message = next_input(&mut host);
        let res_2 = step(&mut host, message, "previous_hash");
        let message = next_input(&mut host);
        let res_3 = step(&mut host, message, "previous_hash");

        assert!(res_1.is_ok());
        assert!(res_2.is_ok());
        assert!(res_3.is_err());

        assert_u64(&mut host, "/tweets/0/likes", Some(1));
    }

    #[test]
    fn transfer_tweet() {
        let state = HostState::default();
        let input_1 = input_1();
        let input_2 = input_transfer();
        let inputs = [input_1.as_slice(), input_2.as_slice()].into_iter();

        let mut host = MockHost::from(state);
        host.as_mut().set_ready_for_input(0);
        host.as_mut().add_next_inputs(0, inputs);

        let message = next_input(&mut host);
        let res_1 = step(&mut host, message, "previous_hash");
        let message = next_input(&mut host);
        let res_2 = step(&mut host, message, "previous_hash");

        assert!(res_1.is_ok());
        assert!(res_2.is_ok());

        assert_not_exists(
            &mut host,
            "/accounts/tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv/tweets/owned/0",
        );
        assert_exist(
            &mut host,
            "/accounts/tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv/tweets/written/0",
        );
        assert_exist(
            &mut host,
            "/accounts/tz1TGu6TN5GSez2ndXXeDX6LgUDvLzPLqgYV/tweets/owned/0",
        );
        assert_not_exists(
            &mut host,
            "/accounts/tz1TGu6TN5GSez2ndXXeDX6LgUDvLzPLqgYV/tweets/writte/0",
        );
    }
}
