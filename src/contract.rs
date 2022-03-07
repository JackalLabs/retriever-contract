#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Coin, Uint128, StdError};
use cw2::set_contract_version;
use cw_utils::{ NativeBalance };

use crate::error::ContractError;
use crate::msg::{OwnerResponse, BlocksResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, JNS, Name};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ibc_name_service";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        blocks_per_year: msg.blocks_per_year,
        owner: info.sender.clone(),

        //prices to register per character count
        cost_for_6: {
            match msg.cost_for_6 {
                Some(x) => {x},
                None => {156250}
            }
        },
        cost_for_5: {
            match msg.cost_for_5 {
                Some(x) => {x},
                None => {312500}
            }
        },
        cost_for_4: {
            match msg.cost_for_4 {
                Some(x) => {x},
                None => {625000}
            }
        },
        cost_for_3: {
            match msg.cost_for_3 {
                Some(x) => {x},
                None => {1250000}
            }
        },
        cost_for_2: {
            match msg.cost_for_2 {
                Some(x) => {x},
                None => {2500000}
            }
        },
        cost_for_1: {
            match msg.cost_for_1 {
                Some(x) => {x},
                None => {5000000}
            }
        },
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("blocks_per_year", msg.blocks_per_year.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetBlocksPerYear { blocks_per_year} => try_set_blocks_per_year(deps, info, blocks_per_year),
        ExecuteMsg::SetOwner { owner } => try_set_owner(deps, info, owner),
        ExecuteMsg::RegisterName { name, years , avatar_url, website, email, twitter, telegram, discord, instagram, reddit} => try_register_name(deps, env, info, name, years, avatar_url, website, email, twitter, telegram, discord, instagram, reddit),

    }
}

pub fn try_register_name(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    name: String, 
    years: u32, 
    avatar_url: Option<String>, 
    website: Option<String>, 
    email: Option<String>,
    twitter: Option<String>, 
    telegram: Option<String>, 
    discord: Option<String>, 
    instagram: Option<String>, 
    reddit: Option<String>
) -> Result<Response, ContractError> {

    // load and save with extra key argument
    let store = deps.storage;

    let char_count = name.chars().count();


    let state = STATE.load(store).unwrap();

    let current_time = env.block.height;
    let existing_name = JNS.may_load(store, &name.clone())?;    // checks if the user is able to register the name
    match existing_name {
        Some(x) => {
            if x.expires > current_time {
                return Err(ContractError::Unauthorized {});
            }
        }
        None => {}
    }

    let mut cost = state.cost_for_6;

    match char_count {
        1 => {
            cost = state.cost_for_1;
        },
        2 => {
            cost = state.cost_for_2;
        },
        3 => {
            cost = state.cost_for_3;
        },
        4 => {
            cost = state.cost_for_4;
        },
        5 => {
            cost = state.cost_for_5;
        },
        _ => {
            cost = state.cost_for_6;
        }
    }

    let total_cost = cost * years;

    let funds = NativeBalance(info.funds);
    let passes = funds.has(&Coin {denom: String::from("ujuno"), amount: Uint128::from(total_cost)});
    if !passes {
        return Err(ContractError::Unauthorized {});
    }

    

    let expiration_date = current_time + ( state.blocks_per_year * years as u64) ; // creates the name data
    let data = Name { 
        id: name.clone(), 
        expires: expiration_date, 
        owner: info.sender, 
        avatar_url: avatar_url, 
        website: website, 
        email: email, 
        twitter: twitter, 
        telegram: telegram, 
        discord: discord, 
        instagram: instagram, 
        reddit: reddit 
    };

    

    JNS.save(store, &name.clone(), &data)?;

    Ok(
        Response::new().add_attribute("method", "try_register_name")
        .add_attribute("tokens_used", total_cost.to_string())
        .add_attribute("name_registered", name)
        .add_attribute("data_accepted", data)
    )
}

pub fn try_set_blocks_per_year(deps: DepsMut, info: MessageInfo, blocks_per_year: u64) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.blocks_per_year = blocks_per_year;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn try_set_owner(deps: DepsMut, info: MessageInfo, owner: Addr) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.owner = owner;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBlocksPerYear {} => to_binary(&query_blocks_per_year(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    }
}

fn query_blocks_per_year(deps: Deps) -> StdResult<BlocksResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(BlocksResponse { blocks_per_year: state.blocks_per_year })
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, BlockInfo};

    const INT_MSG: InstantiateMsg = InstantiateMsg { 
        blocks_per_year: 5048093, 
        cost_for_6: Some(1), 
        cost_for_5: Some(2), 
        cost_for_4: Some(4), 
        cost_for_3: Some(8), 
        cost_for_2: Some(16), 
        cost_for_1: Some(32),
    };

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = INT_MSG.clone();
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBlocksPerYear {}).unwrap();
        let value: BlocksResponse = from_binary(&res).unwrap();
        assert_eq!(5048093, value.blocks_per_year);
    }

    #[test]
    fn change_block_count() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = INT_MSG.clone();
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::SetBlocksPerYear { blocks_per_year: 5048094 };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBlocksPerYear {}).unwrap();
        let value: BlocksResponse = from_binary(&res).unwrap();
        assert_eq!(5048094, value.blocks_per_year);
    }

    #[test]
    fn change_owner() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = INT_MSG.clone();
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::SetOwner { owner: Addr::unchecked("anyone") };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::SetOwner { owner: Addr::unchecked("anyone") };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(Addr::unchecked("anyone"), value.owner);
    }

    #[test]
    fn register_name() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = INT_MSG.clone();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 2 , avatar_url: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let res1 = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let auth_info = mock_info("bobby", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 3 , avatar_url: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let res2 = execute(deps.as_mut(), mock_env(), auth_info, msg);
        
        assert_eq!(res2.is_err(), true);

        println!("{:?}", res1);
        println!("{:?}", res2);
    }
}
