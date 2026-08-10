use soroban_sdk::{contracttype, Address, Env};

use crate::errors::Error;

/// Keys for contract storage.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// The employer / admin address.
    Admin,
    /// The payout token (e.g. a USDC Stellar Asset Contract address).
    Token,
    /// A worker's configured salary, keyed by their address.
    Salary(Address),
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

pub fn set_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::Token, token);
}

pub fn get_token(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Token)
        .ok_or(Error::NotInitialized)
}

pub fn set_salary(env: &Env, worker: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Salary(worker.clone()), &amount);
}

pub fn get_salary(env: &Env, worker: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Salary(worker.clone()))
        .unwrap_or(0)
}
