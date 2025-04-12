use soroban_sdk::{contracttype, Address, Bytes};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    SoroswapContract,
    AquaAmmRouterId,
}
