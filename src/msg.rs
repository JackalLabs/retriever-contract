use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;

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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner {},
    GetBlocksPerYear {},
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
