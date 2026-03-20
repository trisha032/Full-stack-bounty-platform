#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Vec, symbol_short,
};

#[contract]
pub struct BountyContract;

#[derive(Clone)]
#[contracttype]
pub struct Bounty {
    pub creator: Address,
    pub description: String,
    pub reward: i128,
    pub completed: bool,
    pub hunter: Option<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Bounty(u32),
    Count,
}

#[contractimpl]
impl BountyContract {

    // Create a new bounty
    pub fn create_bounty(env: Env, creator: Address, description: String, reward: i128) {
        creator.require_auth();

        let key = DataKey::Count;
        let mut count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        count += 1;

        let bounty = Bounty {
            creator: creator.clone(),
            description,
            reward,
            completed: false,
            hunter: None,
        };

        env.storage().instance().set(&DataKey::Bounty(count), &bounty);
        env.storage().instance().set(&key, &count);
    }

    // Complete a bounty
    pub fn complete_bounty(env: Env, bounty_id: u32, hunter: Address) {
        hunter.require_auth();

        let mut bounty: Bounty = env
            .storage()
            .instance()
            .get(&DataKey::Bounty(bounty_id))
            .expect("Bounty not found");

        if bounty.completed {
            panic!("Already completed");
        }

        bounty.completed = true;
        bounty.hunter = Some(hunter);

        env.storage().instance().set(&DataKey::Bounty(bounty_id), &bounty);
    }

    // Get bounty details
    pub fn get_bounty(env: Env, bounty_id: u32) -> Bounty {
        env.storage()
            .instance()
            .get(&DataKey::Bounty(bounty_id))
            .expect("Bounty not found")
    }
}