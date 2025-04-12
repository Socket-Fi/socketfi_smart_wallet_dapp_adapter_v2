soroban_sdk::contractimport!(
    file =
        "../soroswap_contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.wasm"
);

pub type SoroswapClient<'a> = Client<'a>;
