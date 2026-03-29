#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, symbol_short};

#[contract]
pub struct AgriSubsidyManager;

#[contractimpl]
impl AgriSubsidyManager {

    // Initialize contract with admin
    pub fn init(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
    }

    // Release subsidy (only admin)
    pub fn release(env: Env, farmer: Address, amount: i128) {
        let admin: Address = env.storage().instance()
            .get(&symbol_short!("ADMIN"))
            .unwrap();

        admin.require_auth();

        let existing: i128 = env.storage()
            .instance()
            .get(&farmer)
            .unwrap_or(0);

        let new_amount = existing + amount;

        env.storage().instance().set(&farmer, &new_amount);

        // Emit event
        env.events().publish(
            (symbol_short!("RELEASE"), farmer.clone()),
            new_amount
        );
    }

    // Farmer claims subsidy
    pub fn claim(env: Env, farmer: Address) -> i128 {
        farmer.require_auth();

        let amount: i128 = env.storage()
            .instance()
            .get(&farmer)
            .unwrap_or(0);

        if amount <= 0 {
            panic!("No subsidy available");
        }

        // Reset after claim
        env.storage().instance().set(&farmer, &0);

        env.events().publish(
            (symbol_short!("CLAIM"), farmer.clone()),
            amount
        );

        amount
    }

    // View subsidy balance
    pub fn get_subsidy(env: Env, farmer: Address) -> i128 {
        env.storage()
            .instance()
            .get(&farmer)
            .unwrap_or(0)
    }
}