use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Error, Vec};

use crate::{
    access::{read_aqua_amm_router, read_soroswap_id, write_aqua_amm_router, write_soroswap_id},
    aqua_amm::AquaAmmClient,
    error::ContractError,
    soroswap::SoroswapClient,
};

pub trait AccountMasterTrait {
    // fn initialize(e: Env, admin: Address) -> Result<(), ContractError>;
    fn swap_exact_soroswap(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    );
    fn swap_chain_aqua(
        e: Env,
        to: Address,
        swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        token_in: Address,
        in_amount: u128,
        out_min: u128,
    ) -> u128;
    fn setup_soroswap_id(e: Env, soroswap_id: Address);
    fn setup_aqua_amm_router_id(e: Env, aqua_amm_router_id: Address);
    fn get_pair_router_soroswap(e: Env, token_a: Address, token_b: Address) -> Address;

    fn get_soroswap_id(e: Env) -> Address;
    fn get_aqua_amm_router_id(e: Env) -> Address;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct AccountMaster;

#[contractimpl]
impl AccountMasterTrait for AccountMaster {
    // fn initialize(e: Env, admin: Address) -> Result<(), ContractError> {
    //     let is_initialized = has_admin(&e);
    //     if is_initialized {
    //         return Err(ContractError::AlreadyInitialized);
    //     }
    //     write_admin(&e, &admin);
    //     Ok(())
    // }

    fn swap_exact_soroswap(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) {
        to.require_auth();

        let soroswap_id = read_soroswap_id(&e).unwrap();
        let soroswap_contract = SoroswapClient::new(&e, &soroswap_id);

        soroswap_contract.swap_exact_tokens_for_tokens(
            &amount_in,
            &amount_out_min,
            &path,
            &to,
            &deadline,
        );
    }

    fn swap_chain_aqua(
        e: Env,
        to: Address,
        swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        token_in: Address,
        in_amount: u128,
        out_min: u128,
    ) -> u128 {
        to.require_auth();

    

        let aqua_amm_router_id = read_aqua_amm_router(&e).unwrap();
        let aqua_amm_contract = AquaAmmClient::new(&e, &aqua_amm_router_id);

        aqua_amm_contract.swap_chained(&to, &swaps_chain, &token_in, &in_amount, &out_min)
    }

    fn setup_soroswap_id(e: Env, soroswap_id: Address) {
        // authenticate_admin(&e);
        write_soroswap_id(&e, &soroswap_id);
    }
    fn setup_aqua_amm_router_id(e: Env, aqua_amm_router_id: Address) {
        // authenticate_admin(&e);
        write_aqua_amm_router(&e, &aqua_amm_router_id);
    }

    fn get_pair_router_soroswap(e: Env, token_a: Address, token_b: Address) -> Address {
        let soroswap_id = read_soroswap_id(&e).unwrap();
        let soroswap_contract = SoroswapClient::new(&e, &soroswap_id);

        soroswap_contract.router_pair_for(&token_a, &token_b)
    }

    fn get_soroswap_id(e: Env) -> Address {
        read_soroswap_id(&e).unwrap()
    }
    fn get_aqua_amm_router_id(e: Env) -> Address {
        read_aqua_amm_router(&e).unwrap()
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        // authenticate_admin(&e);
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
