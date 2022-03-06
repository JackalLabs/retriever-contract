use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub blocks_per_year: u64,   // initially 5048093
    pub owner: Addr,            // who owns the contract

    //prices to register a name per character count
    pub cost_for_6: u32,
    pub cost_for_5: u32,
    pub cost_for_4: u32,
    pub cost_for_3: u32,
    pub cost_for_2: u32,
    pub cost_for_1: u32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Name {

    pub id: String,                 // the name itself
    pub expires: u64,               // the block number for which the name expires
    pub owner: Addr,                // the address that owns the name

    // a URL to an avatar image to be assocaited with the name
    pub avatar_url: Option<String>, 

    /// socials ///
    pub website: Option<String>,
    pub email: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub discord: Option<String>,
    pub instagram: Option<String>,
    pub reddit: Option<String>
}

pub const STATE: Item<State> = Item::new("state");
pub const JNS: Map<&str, Name> = Map::new("jns");

