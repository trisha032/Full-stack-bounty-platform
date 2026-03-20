#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_create_bounty() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let bounty_id = client.create_bounty(
        &creator,
        &String::from_str(&env, "Fix Bug #123"),
        &String::from_str(&env, "Fix critical bug in login"),
        &1000i128,
    );

    assert_eq!(bounty_id, 0);
    let bounty = client.get_bounty(&0);
    assert_eq!(bounty.status, BountyStatus::Open);
    assert_eq!(bounty.reward, 1000i128);
}

#[test]
fn test_accept_bounty() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let worker = Address::generate(&env);

    client.create_bounty(
        &creator,
        &String::from_str(&env, "Fix Bug #123"),
        &String::from_str(&env, "Fix critical bug"),
        &500i128,
    );

    client.accept_bounty(&worker, &0);

    let bounty = client.get_bounty(&0);
    assert_eq!(bounty.status, BountyStatus::InProgress);
    assert_eq!(bounty.worker, worker);
}

#[test]
fn test_submit_and_complete_bounty() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let worker = Address::generate(&env);

    client.create_bounty(
        &creator,
        &String::from_str(&env, "Write Docs"),
        &String::from_str(&env, "Write API documentation"),
        &750i128,
    );
    client.accept_bounty(&worker, &0);
    client.submit_work(
        &worker,
        &0,
        &String::from_str(&env, "https://github.com/user/docs"),
    );

    let bounty = client.get_bounty(&0);
    assert_eq!(bounty.status, BountyStatus::Submitted);
    assert_eq!(
        bounty.submission.unwrap(),
        String::from_str(&env, "https://github.com/user/docs")
    );

    client.complete_bounty(&creator, &0);

    let bounty = client.get_bounty(&0);
    assert_eq!(bounty.status, BountyStatus::Completed);
}

#[test]
fn test_cancel_bounty() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);

    client.create_bounty(
        &creator,
        &String::from_str(&env, "Old Bounty"),
        &String::from_str(&env, "This is outdated"),
        &200i128,
    );
    client.cancel_bounty(&creator, &0);

    let bounty = client.get_bounty(&0);
    assert_eq!(bounty.status, BountyStatus::Cancelled);
}

#[test]
fn test_get_all_bounties() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);

    client.create_bounty(
        &creator,
        &String::from_str(&env, "Bounty 1"),
        &String::from_str(&env, "First"),
        &100i128,
    );
    client.create_bounty(
        &creator,
        &String::from_str(&env, "Bounty 2"),
        &String::from_str(&env, "Second"),
        &200i128,
    );
    client.create_bounty(
        &creator,
        &String::from_str(&env, "Bounty 3"),
        &String::from_str(&env, "Third"),
        &300i128,
    );

    let bounties = client.get_all_bounties();
    assert_eq!(bounties.len(), 3);
}

#[test]
#[should_panic(expected = "already accepted")]
fn test_cannot_accept_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let worker1 = Address::generate(&env);
    let worker2 = Address::generate(&env);

    client.create_bounty(
        &creator,
        &String::from_str(&env, "Bug Fix"),
        &String::from_str(&env, "Fix this"),
        &500i128,
    );
    client.accept_bounty(&worker1, &0);
    client.accept_bounty(&worker2, &0); // Should panic
}

#[test]
#[should_panic(expected = "not found")]
fn test_get_nonexistent_bounty() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    client.get_bounty(&999);
}
