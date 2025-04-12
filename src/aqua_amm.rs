soroban_sdk::contractimport!(
    file =
        "../aqua_soroban_amm/target/wasm32-unknown-unknown/release/soroban_liquidity_pool_router_contract.wasm"
);

pub type AquaAmmClient<'a> = Client<'a>;
