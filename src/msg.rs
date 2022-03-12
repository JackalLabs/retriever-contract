use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use crate::state::{ Name, Operator } ;

//REQUIRED BY CW721
use cw_utils::Expiration;
use cosmwasm_std::Binary;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub blocks_per_year: u64,
    pub meta_url: String,
    pub denom: String,
    pub cost_for_6: Option<u64>,
    pub cost_for_5: Option<u64>,
    pub cost_for_4: Option<u64>,
    pub cost_for_3: Option<u64>,
    pub cost_for_2: Option<u64>,
    pub cost_for_1: Option<u64>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetBlocksPerYear { blocks_per_year: u64 },
    SetOwner { owner: Addr },
    RegisterName { 
        name: String, 
        years: u64, 
        avatar_url: Option<String>,
        secret_address: Option<String>, 
        crypto_org_address: Option<String>, 
        starname_address: Option<String>, 
        persistence_address: Option<String>, 
        kava_address: Option<String>,  
        terra_address: Option<String>, 
        website: Option<String>, 
        email: Option<String>, 
        twitter: Option<String>, 
        telegram: Option<String>, 
        discord: Option<String>, 
        instagram: Option<String>, 
        reddit: Option<String>,
    },
    AddTime { name : String, years: u64},
    UpdateParams { 
        name: String, 
        avatar_url: Option<String>, 
        secret_address: Option<String>, 
        crypto_org_address: Option<String>, 
        starname_address: Option<String>, 
        persistence_address: Option<String>, 
        kava_address: Option<String>, 
        terra_address: Option<String>, 
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

    WithdrawBalance{ },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner {},
    GetBlocksPerYear {},
    ResolveName { name : String },
    ResolveAttributes { name : String },
    OwnerOf {
        token_id: String,
    },
    /// List all operators that can access all of the owner's tokens
    /// Return type: `ApprovedForAllResponse`
    ApprovedForAll {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
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

// Name Response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApprovedForAllResponse {
    pub operators: Vec<Operator>,
}

// Name Response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NumTokensResponse {
    pub tokens: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftInfoResponse {
    pub name: String,
    pub description: String,
    pub image: String,
}