use stylus_sdk::{prelude::*, msg};
use alloy_primitives::U256;
use stylus_erc20_example::{MyToken, Erc20Error, InsufficientBalance, InsufficientAllowance};

fn addr(n: u8) -> stylus_erc20_example::Address {
    stylus_erc20_example::Address::from_low_u64_be(n as u64)
}

#[test]
fn test_mint_and_balance() {
    let mut token = MyToken::init_default();
    let user = addr(1);
    token.mint(user, U256::from(1000)).unwrap();
    assert_eq!(token.balance_of(user), U256::from(1000));
    assert_eq!(token.total_supply(), U256::from(1_001_000));
}

#[test]
fn test_transfer_success() {
    let mut token = MyToken::init_default();
    let sender = addr(1);
    let receiver = addr(2);

    msg::set_sender(sender);
    assert!(token.transfer(receiver, U256::from(200)).unwrap());
}

#[test]
fn test_transfer_insufficient_balance() {
    let mut token = MyToken::init_default();
    let sender = addr(1);
    let receiver = addr(2);

    msg::set_sender(sender);
    let err = token.transfer(receiver, U256::from(2_000_000)).unwrap_err();
    match err {
        Erc20Error::InsufficientBalance(_) => (),
        _ => panic!("Expected InsufficientBalance error"),
    }
}

#[test]
fn test_approve_and_allowance() {
    let mut token = MyToken::init_default();
    let owner = addr(1);
    let spender = addr(2);

    msg::set_sender(owner);
    assert!(token.approve(spender, U256::from(300)));
    assert_eq!(token.allowance(owner, spender), U256::from(300));
}

#[test]
fn test_transfer_from_success() {
    let mut token = MyToken::init_default();
    let owner = addr(1);
    let spender = addr(2);
    let receiver = addr(3);

    msg::set_sender(owner);
    token.approve(spender, U256::from(200));

    msg::set_sender(spender);
    assert!(token.transfer_from(owner, receiver, U256::from(150)).unwrap());
    assert_eq!(token.allowance(owner, spender), U256::from(50));
}

#[test]
fn test_burn_tokens() {
    let mut token = MyToken::init_default();
    let user = addr(1);

    msg::set_sender(user);
    token.burn(user, U256::from(500_000)).unwrap();
    assert_eq!(token.balance_of(user), U256::from(500_000));
}
