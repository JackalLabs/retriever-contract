use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Name {
    pub id: String,
    pub expires: u64,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
pub const JNS: Map<&str, Name> = Map::new("jns");

