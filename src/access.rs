use soroban_sdk::{Address, Env};

use crate::data::DataKey;

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

pub fn authenticate_admin(e: &Env) {
    let admin = read_admin(e).unwrap();
    admin.require_auth();
}

pub fn read_soroswap_id(e: &Env) -> Option<Address> {
    let key = DataKey::SoroswapContract;
    e.storage()
        .instance()
        .get(&key)
        .expect("Contract not found!")
}

pub fn write_soroswap_id(e: &Env, soroswap_id: &Address) {
    let key = DataKey::SoroswapContract;
    e.storage().instance().set(&key, soroswap_id);
}

pub fn read_aqua_amm_router(e: &Env) -> Option<Address> {
    let key = DataKey::AquaAmmRouterId;
    e.storage()
        .instance()
        .get(&key)
        .expect("Contract not found!")
}

pub fn write_aqua_amm_router(e: &Env, aqua_amm_router_id: &Address) {
    let key = DataKey::AquaAmmRouterId;
    e.storage().instance().set(&key, aqua_amm_router_id);
}
