#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct UserScore {
    pub score: u32,
}

#[contract]
pub struct CreditScoreContract;

#[contractimpl]
impl CreditScoreContract {

    // Initialize contract (set admin)
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&Symbol::short("ADMIN"), &admin);
    }

    // Set credit score (only admin)
    pub fn set_score(env: Env, user: Address, score: u32) {
        let admin: Address = env.storage().instance()
            .get(&Symbol::short("ADMIN"))
            .unwrap();

        admin.require_auth();

        let user_score = UserScore { score };
        env.storage().persistent().set(&user, &user_score);
    }

    // Get credit score
    pub fn get_score(env: Env, user: Address) -> u32 {
        let result: Option<UserScore> = env.storage().persistent().get(&user);

        match result {
            Some(data) => data.score,
            None => 0, // default score
        }
    }
}