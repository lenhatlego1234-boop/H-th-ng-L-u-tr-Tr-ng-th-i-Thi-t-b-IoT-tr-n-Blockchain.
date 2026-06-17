#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct IotDeviceTracker;

#[contractimpl]
impl IotDeviceTracker {

    pub fn set_status(env: Env, device_id: Symbol, is_active: bool) {

        env.storage().instance().set(&device_id, &is_active);
    }

    pub fn get_status(env: Env, device_id: Symbol) -> bool {
        env.storage()
            .instance()
            .get(&device_id)
            .unwrap_or(false)
    }
}
