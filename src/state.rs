use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{ Addr, CanonicalAddr } ;
use cw_storage_plus::{Item, Map};
use cw_utils::Expiration;

pub const OPERATOR_PREFIX: &[u8] = b"operators";


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub blocks_per_year: u64,   // initially 5048093
    pub owner: Addr,            // who owns the contract

    pub meta_url: String,

    pub denom: String,          // accepted token denom

    //prices to register a name per character count
    pub cost_for_6: u64,
    pub cost_for_5: u64,
    pub cost_for_4: u64,
    pub cost_for_3: u64,
    pub cost_for_2: u64,
    pub cost_for_1: u64
}



#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: CanonicalAddr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Name {

    pub id: String,                 // the name itself
    pub expires: u64,               // the block number for which the name expires
    pub owner: Addr,                // the address that owns the name

    pub approvals: Vec<Approval>,   // NFT stuff

    // a URL to an avatar image to be assocaited with the name
    pub avatar_url: Option<String>, 

    // other chains that had to be quirky and get their own coin_id instead of just going with the default and being a pain in my ass
    pub secret_address: Option<String>, 
    pub crypto_org_address: Option<String>,
    pub starname_address: Option<String>,
    pub persistence_address: Option<String>,
    pub kava_address: Option<String>,
    pub terra_address: Option<String>, 

    /// socials ///
    pub website: Option<String>,
    pub email: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub discord: Option<String>,
    pub instagram: Option<String>,
    pub reddit: Option<String>
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name (id: {}, expires: {}, owner: {}, avatar_url: {:?}, website: {:?}, email: {:?}, twitter: {:?}, telegram: {:?}, discord: {:?}, instagram: {:?}, reddit: {:?})", self.id, self.expires, self.owner, self.avatar_url, self.website, self.email, self.twitter, self.telegram, self.discord, self.instagram, self.reddit)
    }
}

impl Into<String> for Name {
    fn into(self: Name) -> String {
        format!("Name (id: {}, expires: {}, owner: {}, avatar_url: {:?}, website: {:?}, email: {:?}, twitter: {:?}, telegram: {:?}, discord: {:?}, instagram: {:?}, reddit: {:?})", self.id, self.expires, self.owner, self.avatar_url, self.website, self.email, self.twitter, self.telegram, self.discord, self.instagram, self.reddit)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Operator {
    pub owner: String,
    pub expires: Expiration,
}

pub const OPERATORS: Map<String, Vec<Operator>> = Map::new("operators");

pub const STATE: Item<State> = Item::new("state");

pub const JNS: Map<&str, Name> = Map::new("jns");

