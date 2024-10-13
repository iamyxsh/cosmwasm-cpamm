use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::Item;
use cw20::Denom;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub reserve: Uint128,
    pub denom: Denom,
}

pub const TOKEN1: Item<Token> = Item::new("tokenA");
pub const TOKEN2: Item<Token> = Item::new("tokenB");

pub const OWNER: Item<Option<Addr>> = Item::new("owner");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Fees {
    pub protocol_fee_recipient: Addr,
    pub protocol_fee_percent: Decimal,
    pub lp_fee_percent: Decimal,
}

pub const FEES: Item<Fees> = Item::new("fees");

pub const FROZEN: Item<bool> = Item::new("frozen");
