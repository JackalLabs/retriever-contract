use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use crate::state::Name;

//REQUIRED BY CW721
use cw_utils::Expiration;
use cosmwasm_std::Binary;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub blocks_per_year: u64,
    pub cost_for_6: Option<u32>,
    pub cost_for_5: Option<u32>,
    pub cost_for_4: Option<u32>,
    pub cost_for_3: Option<u32>,
    pub cost_for_2: Option<u32>,
    pub cost_for_1: Option<u32>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetBlocksPerYear { blocks_per_year: u64 },
    SetOwner { owner: Addr },
    RegisterName { 
        name: String, 
        years: u32, 
        avatar_url: Option<String>, 
        website: Option<String>, 
        email: Option<String>, 
        twitter: Option<String>, 
        telegram: Option<String>, 
        discord: Option<String>, 
        instagram: Option<String>, 
        reddit: Option<String>,
    },
    AddTime { name : String, years: u32},
    UpdateParams { 
        name: String, 
        avatar_url: Option<String>, 
        website: Option<String>, 
        email: Option<String>, 
        twitter: Option<String>, 
        telegram: Option<String>, 
        discord: Option<String>, 
        instagram: Option<String>, 
        reddit: Option<String>,
    },

    /**
     * ALL THE CW721 STANDARD FUNCTIONS
     */
    TransferNft { recipient: String, token_id: String },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        message: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke { spender: String, token_id: String },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner {},
    GetBlocksPerYear {},
    ResolveName { name : String },
    ResolveAttributes { name : String },
}

// Blocks Per Year response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlocksResponse {
    pub blocks_per_year: u64,
}

// Owner Response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: Addr,
}


// Name Response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NameResponse {
    pub name: Name,
}
