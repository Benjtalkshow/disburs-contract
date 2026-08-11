#![no_std]
//! Disburs payroll contract.
//!
//! An employer (admin) funds a treasury held by this contract and pays workers
//! a configured salary in a token (e.g. USDC) on Stellar. This is the basic
//! building block; contract-aware runs, FX, and zero-knowledge privacy (salary
//! commitments + on-chain proof verification, see the README roadmap) come
//! later.

mod errors;
mod storage;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, contractmeta, symbol_short, token, Address, Env};

use crate::errors::Error;

contractmeta!(key = "version", val = "0.1.0");
contractmeta!(
    key = "description",
    val = "Disburs payroll: fund a treasury and pay workers in a token on Stellar."
);
contractmeta!(key = "license", val = "MIT");

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {
    /// Initialize with the employer/admin and the payout token address.
    pub fn __constructor(env: Env, admin: Address, token: Address) {
        storage::set_admin(&env, &admin);
        storage::set_token(&env, &token);
    }

    /// Fund the treasury: transfer `amount` of the token from `from` into the
    /// contract. Anyone can top up, but they must authorize the transfer.
    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        from.require_auth();
        let token = storage::get_token(&env)?;
        token::TokenClient::new(&env, &token).transfer(
            &from,
            &env.current_contract_address(),
            &amount,
        );
        env.events().publish((symbol_short!("deposit"), from), amount);
        Ok(())
    }

    /// Admin-only: set (or update) a worker's salary.
    pub fn set_salary(env: Env, worker: Address, amount: i128) -> Result<(), Error> {
        storage::get_admin(&env)?.require_auth();
        if amount < 0 {
            return Err(Error::InvalidAmount);
        }
        storage::set_salary(&env, &worker, amount);
        Ok(())
    }

    /// Admin-only: pay a worker their configured salary from the treasury.
    pub fn pay(env: Env, worker: Address) -> Result<(), Error> {
        storage::get_admin(&env)?.require_auth();

        let amount = storage::get_salary(&env, &worker);
        if amount <= 0 {
            return Err(Error::NoSalarySet);
        }
        let token = storage::get_token(&env)?;
        let client = token::TokenClient::new(&env, &token);
        let treasury = env.current_contract_address();
        if client.balance(&treasury) < amount {
            return Err(Error::InsufficientTreasury);
        }
        client.transfer(&treasury, &worker, &amount);
        env.events().publish((symbol_short!("pay"), worker), amount);
        Ok(())
    }

    /// Admin-only: pull unused funds back out of the treasury to `to`.
    pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        storage::get_admin(&env)?.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let token = storage::get_token(&env)?;
        let client = token::TokenClient::new(&env, &token);
        let treasury = env.current_contract_address();
        if client.balance(&treasury) < amount {
            return Err(Error::InsufficientTreasury);
        }
        client.transfer(&treasury, &to, &amount);
        env.events().publish((symbol_short!("withdraw"), to), amount);
        Ok(())
    }

    /// Token balance held by the treasury.
    pub fn treasury_balance(env: Env) -> Result<i128, Error> {
        let token = storage::get_token(&env)?;
        Ok(token::TokenClient::new(&env, &token).balance(&env.current_contract_address()))
    }

    /// A worker's configured salary (0 if unset).
    pub fn salary_of(env: Env, worker: Address) -> i128 {
        storage::get_salary(&env, &worker)
    }

    /// The current admin address.
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        storage::get_admin(&env)
    }

    /// Admin-only: hand admin rights to a new address.
    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        storage::get_admin(&env)?.require_auth();
        storage::set_admin(&env, &new_admin);
        Ok(())
    }
}
