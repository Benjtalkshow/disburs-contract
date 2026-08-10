#![cfg(test)]

use soroban_sdk::{testutils::Address as _, token, Address, Env};

use crate::{PayrollContract, PayrollContractClient};

/// Deploy a fresh payroll contract with a test token; mint 1_000 to the admin.
/// Returns (env, admin, contract_id, token_address).
fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(issuer);
    let token_address = sac.address();
    token::StellarAssetClient::new(&env, &token_address).mint(&admin, &1_000);

    let contract_id = env.register(PayrollContract, (admin.clone(), token_address.clone()));
    (env, admin, contract_id, token_address)
}

#[test]
fn deposit_set_salary_and_pay() {
    let (env, admin, contract_id, token_address) = setup();
    let client = PayrollContractClient::new(&env, &contract_id);
    let token = token::TokenClient::new(&env, &token_address);
    let worker = Address::generate(&env);

    client.deposit(&admin, &500);
    assert_eq!(client.treasury_balance(), 500);

    client.set_salary(&worker, &200);
    assert_eq!(client.salary_of(&worker), 200);

    client.pay(&worker);
    assert_eq!(token.balance(&worker), 200);
    assert_eq!(client.treasury_balance(), 300);
}

#[test]
fn pay_without_salary_fails() {
    let (env, _admin, contract_id, _token) = setup();
    let client = PayrollContractClient::new(&env, &contract_id);
    let worker = Address::generate(&env);

    assert!(client.try_pay(&worker).is_err());
}

#[test]
fn pay_with_empty_treasury_fails() {
    let (env, _admin, contract_id, _token) = setup();
    let client = PayrollContractClient::new(&env, &contract_id);
    let worker = Address::generate(&env);

    client.set_salary(&worker, &100);
    assert!(client.try_pay(&worker).is_err());
}
