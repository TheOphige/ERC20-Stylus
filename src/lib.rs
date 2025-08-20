#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

mod erc20;
pub use erc20::*;

use alloy_primitives::U256;
use stylus_sdk::{msg, prelude::*};

pub type Address = stylus_sdk::prelude::Address;

pub struct StylusTokenParams;

impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

#[contract]
pub struct MyToken {
    #[storage]
    pub erc20: ERC20<StylusTokenParams>,
    pub owner: Address,
}

#[external]
impl MyToken {
    pub fn new(initial_supply: U256) -> Self {
        let caller = msg::sender();
        let mut balances = Mapping::new();
        balances.insert(caller, initial_supply);
        evm::log(Transfer { from: Address::ZERO, to: caller, value: initial_supply });

        Self {
            erc20: ERC20 {
                name: StylusTokenParams::NAME.into(),
                symbol: StylusTokenParams::SYMBOL.into(),
                decimals: StylusTokenParams::DECIMALS,
                total_supply: initial_supply,
                balances,
                allowances: Mapping::new(),
                phantom: PhantomData,
            },
            owner: caller,
        }
    }

    pub fn name(&self) -> String { self.erc20.name.clone() }
    pub fn symbol(&self) -> String { self.erc20.symbol.clone() }
    pub fn decimals(&self) -> u8 { self.erc20.decimals }
    pub fn total_supply(&self) -> U256 { self.erc20.total_supply }
    pub fn balance_of(&self, owner: Address) -> U256 { self.erc20.balances.get(owner).unwrap_or(U256::zero()) }
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowances.get((owner, spender)).unwrap_or(U256::zero())
    }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self.erc20._transfer(msg::sender(), to, value)?;
        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.erc20._approve(msg::sender(), spender, value);
        true
    }

    pub fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<bool, Erc20Error> {
        let sender = msg::sender();
        let allowance = self.erc20.allowances.get((from, sender)).unwrap_or(U256::zero());
        if allowance < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: sender,
                have: allowance,
                want: value,
            }));
        }
        self.erc20.allowances.insert((from, sender), allowance - value);
        self.erc20._transfer(from, to, value)?;
        Ok(true)
    }

    pub fn mint(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        if msg::sender() != self.owner {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from: msg::sender(),
                have: U256::zero(),
                want: value,
            }));
        }
        let balance = self.erc20.balances.get(to).unwrap_or(U256::zero());
        self.erc20.balances.insert(to, balance + value);
        self.erc20.total_supply += value;
        evm::log(Transfer { from: Address::ZERO, to, value });
        Ok(())
    }

    pub fn burn(&mut self, from: Address, value: U256) -> Result<(), Erc20Error> {
        let balance = self.erc20.balances.get(from).unwrap_or(U256::zero());
        if balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: balance,
                want: value,
            }));
        }
        self.erc20.balances.insert(from, balance - value);
        self.erc20.total_supply -= value;
        evm::log(Transfer { from, to: Address::ZERO, value });
        Ok(())
    }
}
