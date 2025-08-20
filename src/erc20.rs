// Imported packages
use alloc::string::String;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use core::marker::PhantomData;
use stylus_sdk::{
    evm,
    msg,
    prelude::*,
};

pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}

sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

#[derive(SolidityError)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

#[storage]
pub struct ERC20<T> {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U256,
    pub balances: Mapping<Address, U256>,
    pub allowances: Mapping<(Address, Address), U256>,
    pub phantom: PhantomData<T>,
}

impl<T: Erc20Params> ERC20<T> {
    pub fn _transfer(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<(), Erc20Error> {
        let sender_balance = self.balances.get(from).unwrap_or(U256::zero());
        if sender_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: sender_balance,
                want: value,
            }));
        }
        self.balances.insert(from, sender_balance - value);
        let recipient_balance = self.balances.get(to).unwrap_or(U256::zero());
        self.balances.insert(to, recipient_balance + value);
        evm::log(Transfer { from, to, value });
        Ok(())
    }

    pub fn _approve(&mut self, owner: Address, spender: Address, value: U256) {
        self.allowances.insert((owner, spender), value);
        evm::log(Approval { owner, spender, value });
    }
}
