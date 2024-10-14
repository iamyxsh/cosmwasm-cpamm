use cosmwasm_std::Uint128;

pub const CONTRACT_NAME: &str = "crates.io:product-amm";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const FEE_SCALE_FACTOR: Uint128 = Uint128::new(10_000);
pub const MAX_FEE_PERCENT: &str = "10";
pub const FEE_DECIMAL_PRECISION: Uint128 = Uint128::new(10u128.pow(20));
