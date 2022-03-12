#[cfg(not(feature = "library"))]
use cosmwasm_std::{Timestamp, entry_point, BankMsg, from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Coin, Uint128, StdError, CosmosMsg, CanonicalAddr};
use cw2::set_contract_version;
use cw_utils::{ NativeBalance };

use crate::error::ContractError;
use crate::msg::{NftInfoResponse, ContractInfoResponse, NumTokensResponse, ApprovedForAllResponse, OwnerResponse, BlocksResponse, NameResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, OPERATORS, Operator, STATE, JNS, Name, Approval};

use cw_utils::Expiration;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ibc_name_service";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SECONDS_IN_YEAR: u64 = 365 * 24 * 60 * 60;

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
        meta_url: msg.meta_url.to_string(),
        denom: msg.denom.to_string(),
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
        ExecuteMsg::RegisterName { name, years , avatar_url, terra_address, secret_address, crypto_org_address, starname_address, persistence_address, kava_address, website, email, twitter, telegram, discord, instagram, reddit} => try_register_name(deps, env, info, name, years, avatar_url, terra_address, secret_address, crypto_org_address, starname_address, persistence_address, kava_address, website, email, twitter, telegram, discord, instagram, reddit),
        ExecuteMsg::AddTime { name, years} => try_add_time(deps, env, info, name, years),
        ExecuteMsg::UpdateParams { name, avatar_url, terra_address, secret_address, crypto_org_address, starname_address, persistence_address, kava_address, website, email, twitter, telegram, discord, instagram, reddit} => try_update_name(deps, env, info, name, avatar_url, terra_address, secret_address, crypto_org_address, starname_address, persistence_address, kava_address, website, email, twitter, telegram, discord, instagram, reddit),
        ExecuteMsg::TransferNft {recipient, token_id} => transfer_nft (deps, env, info, recipient, token_id),
        ExecuteMsg::SendNft {contract, token_id, message} => try_send_nft (deps, env, info, contract, token_id, message),
        ExecuteMsg::Approve {spender, token_id, expires} => handle_approve (deps, env, info, spender, token_id, expires),
        ExecuteMsg::Revoke {spender, token_id} => handle_revoke (deps, env, info, spender, token_id),
        ExecuteMsg::ApproveAll {operator, expires} => handle_approve_all (deps, env, info, operator, expires),
        ExecuteMsg::RevokeAll {operator} => handle_revoke_all (deps, env, info, operator),
        ExecuteMsg::WithdrawBalance {} => handle_withdraw_balance(deps, env, info),

    }
}

pub fn handle_withdraw_balance(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let state = STATE.load(deps.storage)?;


    if info.sender != state.owner {
        return Err(ContractError::Std(StdError::generic_err(
            "Not authorized to withdraw payment.",
        )));
    }

    // get balance and send all to recipient
    let balance = deps.querier.query_all_balances(env.contract.address)?;
    let send = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: balance.clone(),
    };

    let data_msg = format!("{:?}", balance).into_bytes();

    Ok(Response::new()
        .add_message(send)
        .add_attribute("action", "withdraw")
        .add_attribute("payed_to", info.sender.to_string())
        .set_data(data_msg))
    }

pub fn handle_approve_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {

    let expires = expires.unwrap_or_default();

    if expires.is_expired(&env.block) {
        return Err(ContractError::Std(StdError::generic_err(
            "Cannot set approval that is already expired",
        )));
    }

    let op = Operator {
        owner: operator.clone(),
        expires: expires,
    };

    let mut ops = OPERATORS.load(deps.storage, info.sender.to_string()).unwrap_or(vec![]);
    ops.push(op);
    OPERATORS.save(deps.storage, info.sender.to_string(), &ops)?;


    Ok(Response::new().add_attribute("action", "approve_all").add_attribute("sender", info.sender).add_attribute("operator", operator))
}

pub fn handle_revoke_all(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator: String,
) -> Result<Response, ContractError> {

    OPERATORS.remove(deps.storage, info.sender.to_string());


    Ok(Response::new().add_attribute("action", "approve_all").add_attribute("sender", info.sender).add_attribute("operator", operator))
}

pub fn handle_revoke(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    token_id: String,
) -> Result<Response, ContractError> {

    let approve = _update_approvals(deps, env, info.sender.to_string(), spender.clone(), token_id.clone(), false, None);

    if approve.is_err() {
        return Err(ContractError::Std(StdError::generic_err(approve.unwrap_err().to_string())));
    }

    Ok(Response::new().add_attribute("action", "revoke").add_attribute("sender", info.sender).add_attribute("spender", spender).add_attribute("token_id", token_id))
}

pub fn handle_approve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    token_id: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {

    let approve = _update_approvals(deps, env, info.sender.to_string(), spender.clone(), token_id.clone(), true, expires);

    if approve.is_err() {
        return Err(ContractError::Std(StdError::generic_err(approve.unwrap_err().to_string())));
    }

    Ok(Response::new().add_attribute("action", "approve").add_attribute("sender", info.sender).add_attribute("spender", spender).add_attribute("token_id", token_id))
}

pub fn _update_approvals(
    deps: DepsMut,
    env: Env,
    sender: String,
    spender: String,
    token_id: String,
    add: bool,
    expires: Option<Expiration>,
) -> Result<NameResponse, ContractError> {
    let token = JNS.may_load(deps.storage, &token_id);

    if token.is_err() {
        return Err(ContractError::Std(StdError::generic_err(token.unwrap_err().to_string())));
    }

    let mut token = token.unwrap().unwrap();

    if token.owner != sender {
        return Err(ContractError::Unauthorized {});
    }


    // update the approval list (remove any for the same spender before adding)
    let spender_raw = deps.api.addr_canonicalize(&spender)?;

    token.approvals = token
        .approvals
        .into_iter()
        .filter(|apr| apr.spender != spender_raw)
        .collect();

    // only difference between approve and revoke
    if add {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Std(StdError::generic_err(
                "Cannot set approval that is already expired",
            )));
        };
        let approval = Approval {
            spender: spender_raw,
            expires,
        };
        token.approvals.push(approval);
    }

    JNS.save(deps.storage, &token_id, &token)?;

    Ok(NameResponse { name: token })
}

pub fn try_send_nft (
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    contract: String,
    token_id: String,
    msg: Binary
) -> Result<Response, ContractError> {
    
    // Unwrap message first
    let _msgs: Vec<CosmosMsg> = vec![from_binary(&msg)?];

    // Transfer token
    let res = _try_transfer_nft(deps, env, info.clone(), contract.clone(), token_id.clone());

    if res.is_err() {
        return Err(ContractError::Std(StdError::generic_err(res.unwrap_err().to_string())));
    }

    // Send message
    Ok(Response::new().add_attribute("action", "send_nft").add_attribute("sender", info.sender).add_attribute("recipient", contract).add_attribute("token_id", token_id))

}

pub fn transfer_nft (
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    recipient: String,
    token_id: String
) -> Result<Response, ContractError> {
    
    _try_transfer_nft(deps, env, info, recipient.clone(), token_id.clone())?;

    Ok(
        Response::new().add_attribute("method", "try_transfer_nft")
        .add_attribute("name_transfered", token_id)
        .add_attribute("new_owner", recipient)
    )
}

fn check_can_send(
    op: Vec<Operator>,
    sraw: CanonicalAddr,
    sender_raw: &Addr,
    env: &Env,
    _info: &MessageInfo,
    token: Name,
) -> Result<(), ContractError> {
    // owner can send

    if &token.owner == sender_raw {
        return Ok(());
    }


    // any non-expired token approval can send
    if token
        .approvals
        .iter()
        .any(|apr| apr.spender == sraw && !apr.expires.is_expired(&env.block))
    {
        return Ok(());
    }

    // operator can send
    for o in op {
        if &o.owner == sender_raw {
            if !o.expires.is_expired(&env.block) {
                return Ok(());
            }
        }
    }

    Err(ContractError::Unauthorized {})
    
}

pub fn _try_transfer_nft (
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    recipient: String,
    token_id: String
) -> Result<Response, ContractError> {

    let store = deps.storage;
    let existing_name = JNS.may_load(store, &token_id.clone())?;    // checks if the user is able to register the name
    if existing_name == None {
        return Err(ContractError::Std(StdError::not_found("Name not registered.")));
    }

    let real_name = existing_name.unwrap();
    let checked= deps.api.addr_validate(&recipient);

    let sender_raw = &info.sender;
    let sraw = deps.api.addr_canonicalize(&sender_raw.to_string())?;
    let op = OPERATORS.may_load(store, real_name.owner.to_string())?;

    let r = check_can_send(op.unwrap_or(vec![]), sraw, sender_raw, &env, &info, real_name.clone());

    if r.is_err() {
        return Err(ContractError::Unauthorized{});
    }

    if checked.is_err() {
        return Err(ContractError::Std(StdError::generic_err("Recipient is not a valid address.")));
    }

    let address = checked.unwrap();
    
    let new_name = Name {
        id: real_name.id,
        expires: real_name.expires,
        owner: address.clone(),
        approvals: vec![],
        avatar_url: None,
        terra_address: None,
        secret_address: None,
        crypto_org_address: None,
        starname_address: None,
        persistence_address: None,
        kava_address: None,
        website: None,
        email: None,
        twitter: None,
        telegram: None,
        discord: None,
        instagram: None,
        reddit: None
    };

    JNS.save(store, &token_id.clone(), &new_name)?;

    Ok(
        Response::new().add_attribute("method", "try_transfer_nft")
        .add_attribute("name_transfered", token_id)
        .add_attribute("new_owner", address.to_string())
    )
}


pub fn try_add_time(
    deps: DepsMut, 
    _env: Env, 
    info: MessageInfo, 
    name: String, 
    years: u64
)-> Result<Response, ContractError> {
    let store = deps.storage;

    let existing_name = JNS.may_load(store, &name.clone())?;    // checks if the user is able to register the name
    if existing_name == None {
        return Err(ContractError::Unauthorized {});
    }

    let mut real_name = existing_name.unwrap();

    if real_name.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let char_count = name.chars().count();

    let state = STATE.load(store).unwrap();
    let mut _cost = state.cost_for_6;

    match char_count {
        1 => {
            _cost = state.cost_for_1;
        },
        2 => {
            _cost = state.cost_for_2;
        },
        3 => {
            _cost = state.cost_for_3;
        },
        4 => {
            _cost = state.cost_for_4;
        },
        5 => {
            _cost = state.cost_for_5;
        },
        _ => {
            _cost = state.cost_for_6;
        }
    }

    let total_cost = _cost * years;

    let funds = NativeBalance(info.funds);
    let passes = funds.has(&Coin {denom: String::from(state.denom), amount: Uint128::from(total_cost)});
    if !passes {
        return Err(ContractError::Unauthorized {});
    }

    real_name.expires = real_name.expires + ( state.blocks_per_year * years as u64 );

    JNS.save(store, &name.clone(), &real_name)?;

    Ok(
        Response::new().add_attribute("method", "try_register_name")
        .add_attribute("tokens_used", total_cost.to_string())
        .add_attribute("name_registered", name)
        .add_attribute("data_accepted", real_name)
    )
}
pub fn try_update_name(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    name: String, 
    avatar_url: Option<String>, 
    terra_address: Option<String>, 
    secret_address: Option<String>, 
    crypto_org_address: Option<String>, 
    starname_address: Option<String>, 
    persistence_address: Option<String>, 
    kava_address: Option<String>, 
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

    let current_time = env.block.time.nanos();
    let existing_name = JNS.may_load(store, &name.clone());    // checks if the user is able to register the name
    if existing_name.is_err() {
        return Err(ContractError::Std(StdError::not_found("Name not register.")));
    }

    let existing_name = existing_name.unwrap().unwrap();

    if existing_name.expires < current_time {
        return Err(ContractError::Std(StdError::not_found("Name not register.")));
    }

    if existing_name.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }




    let data = Name { 
        id: existing_name.id, 
        expires: existing_name.expires, 
        owner: existing_name.owner, 
        approvals: vec![],
        avatar_url: avatar_url, 
        terra_address: terra_address,
        secret_address: secret_address,
        crypto_org_address: crypto_org_address,
        starname_address: starname_address,
        persistence_address: persistence_address,
        kava_address: kava_address,
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
        .add_attribute("name_updated", name)
        .add_attribute("data_accepted", data)
    )
}

pub fn try_register_name(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    name: String, 
    years: u64, 
    avatar_url: Option<String>, 
    terra_address: Option<String>, 
    secret_address: Option<String>, 
    crypto_org_address: Option<String>, 
    starname_address: Option<String>, 
    persistence_address: Option<String>, 
    kava_address: Option<String>, 
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

    let current_time = env.block.time.nanos();

    let existing_name = JNS.may_load(store, &name.clone())?;    // checks if the user is able to register the name
    match existing_name {
        Some(x) => {
            if x.expires > current_time {
                return Err(ContractError::Std(StdError::generic_err("Name is already registered.")));
            }
        }
        None => {}
    }

    let mut _cost = state.cost_for_6;

    match char_count {
        1 => {
            _cost = state.cost_for_1;
        },
        2 => {
            _cost = state.cost_for_2;
        },
        3 => {
            _cost = state.cost_for_3;
        },
        4 => {
            _cost = state.cost_for_4;
        },
        5 => {
            _cost = state.cost_for_5;
        },
        _ => {
            _cost = state.cost_for_6;
        }
    }

    let total_cost = _cost * years;

    let funds = NativeBalance(info.funds);
    let passes = funds.has(&Coin {denom: String::from(state.denom), amount: Uint128::from(total_cost)});

    if !passes {
        return Err(ContractError::Std(StdError::generic_err(format!("Not enough juno being sent. Wanted: {}", total_cost))));
    }

    

    let expiration_date = current_time + ( Timestamp::from_seconds(SECONDS_IN_YEAR * years).nanos()) ; // creates the name data
    let data = Name { 
        id: name.clone(), 
        expires: expiration_date, 
        owner: info.sender, 
        approvals: vec![],
        avatar_url: avatar_url, 
        terra_address: terra_address,
        secret_address: secret_address, 
        crypto_org_address: crypto_org_address, 
        starname_address: starname_address, 
        persistence_address: persistence_address, 
        kava_address: kava_address, 
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
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBlocksPerYear {} => to_binary(&query_blocks_per_year(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::ResolveName { name } => to_binary(&query_name_owner(deps, env, name)?),
        QueryMsg::ResolveAttributes { name } => to_binary(&query_name_attributes(deps, env, name)?),
        QueryMsg::OwnerOf { token_id } => to_binary(&query_name_owner(deps, env, token_id)?),
        QueryMsg::ApprovedForAll {
            owner,
            start_after,
            limit,
        } => to_binary(&query_all_approvals(deps, owner, start_after, limit)?),
        QueryMsg::NumTokens {} => to_binary(&query_num_tokens()?),
        QueryMsg::ContractInfo {} => to_binary(&query_contract_info()?),
        QueryMsg::NftInfo { token_id } => to_binary(&query_nft_info(deps, env, token_id)?),
    }
}

fn query_nft_info( deps: Deps, env:Env, token_id: String ) -> StdResult<NftInfoResponse> {

    let exists = JNS.may_load(deps.storage, &token_id);
    if exists.is_err() {
        return Err(StdError::NotFound { kind: "Name is not registered.".to_string()});
    }

    let ret_name = exists.unwrap().unwrap();

    if ret_name.expires <= env.block.time.nanos() {
        return Err(StdError::NotFound { kind: "Name is not registered.".to_string()});
    }

    let state = STATE.load(deps.storage)?;


    Ok(NftInfoResponse {
        name: format!("{}.rns", token_id.clone()),
        description: "An IBC Name Resolver living on the JUNO network.".to_string(),
        image: format!("{}/{}", state.meta_url, token_id),
    })
} 


fn query_contract_info() -> StdResult<ContractInfoResponse> {
    Ok(ContractInfoResponse {
        name: "JACKAL Name Service".to_string(),
        symbol: "RNS".to_string(),
    })
} 

fn query_num_tokens() -> StdResult<NumTokensResponse> {
    Ok(NumTokensResponse {tokens: 0})
} 

fn query_all_approvals(
    deps: Deps,
    owner: String,
    _start_after: Option<String>,
    _limit: Option<u32>,
) -> StdResult<ApprovedForAllResponse> {
    
    let ops = OPERATORS.load(deps.storage, owner).unwrap_or(vec![]);

    Ok(ApprovedForAllResponse {operators: ops})

}

fn query_blocks_per_year(deps: Deps) -> StdResult<BlocksResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(BlocksResponse { blocks_per_year: state.blocks_per_year })
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

fn query_name_attributes(deps: Deps, env: Env, name: String) -> StdResult<NameResponse> {
    let exists = JNS.may_load(deps.storage, &name);
    if exists.is_err() {
        return Err(StdError::NotFound { kind: "Name is not registered.".to_string()});
    }

    let ret_name = exists.unwrap().unwrap();

    if ret_name.expires <= env.block.time.nanos() {
        return Err(StdError::NotFound { kind: "Name is not registered.".to_string()});
    }

    Ok(NameResponse { name: ret_name })
}

fn query_name_owner(deps: Deps, env: Env, name: String) -> StdResult<OwnerResponse> {
    let exists = JNS.may_load(deps.storage, &name);
    if exists.is_err() {
        return Err(StdError::NotFound { kind: "Name is not registered.".to_string()});
    }

    let ret_name = exists.unwrap().unwrap();

    if ret_name.expires <= env.block.time.nanos() {
        return Err(StdError::NotFound { kind: "Name is not registered.".to_string()});
    }

    Ok(OwnerResponse { owner: ret_name.owner })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    

    fn int_mgs() -> InstantiateMsg{

        InstantiateMsg { 
            blocks_per_year: 5048093, 
            meta_url: "example.com".to_string(),
            denom: "ujuno".to_string(),
            cost_for_6: Some(1), 
            cost_for_5: Some(2), 
            cost_for_4: Some(4), 
            cost_for_3: Some(8), 
            cost_for_2: Some(16), 
            cost_for_1: Some(32),
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = int_mgs();
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

        let msg = int_mgs();
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

        let msg = int_mgs();
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

        let msg = int_mgs();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 2 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let res1 = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let auth_info = mock_info("bobby", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 3 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let res2 = execute(deps.as_mut(), mock_env(), auth_info, msg);
        
        assert_eq!(res2.is_err(), true);

        println!("{:?}", res1);
        println!("{:?}", res2);
    }

    #[test]
    fn add_time_to_name() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = int_mgs();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 2 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let res1 = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::AddTime { name: String::from("testname") , years: 2 };
        let res2 = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let auth_info = mock_info("bobby", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::AddTime { name: String::from("testname") , years: 2 };
        let res3 = execute(deps.as_mut(), mock_env(), auth_info, msg);
        assert_eq!(res3.is_err(), true);
        
        println!("{:?}", res1);
        println!("{:?}", res2);
        println!("{:?}", res3);
    }

    #[test]
    fn resolve_name() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = int_mgs();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 2 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        
        let res = query(deps.as_ref(), mock_env(), QueryMsg::ResolveName { name : String::from("testname")}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(Addr::unchecked("annie"), value.owner);

    }

    #[test]
    fn request_name() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = int_mgs();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: String::from("testname") , years: 2 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        
        let res = query(deps.as_ref(), mock_env(), QueryMsg::ResolveAttributes { name : String::from("testname")}).unwrap();
        let value: NameResponse = from_binary(&res).unwrap();
        assert_eq!(Name {id: String::from("testname") , expires: 1571797419879305533 + Timestamp::from_seconds(SECONDS_IN_YEAR * 2).nanos() , owner: Addr::unchecked("annie"), approvals: vec![], avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None}, value.name);

    }

    #[test]
    fn transferring_nft() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = int_mgs();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Mint a token
        let token_id = "melt".to_string();

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: token_id.clone() , years: 2 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let _res1 = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // random cannot transfer
        let random = mock_info("bobby", &coins(200000, "ujuno"));
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: "random".into(),
            token_id: token_id.clone(),
        };

        let _err = execute(deps.as_mut(), mock_env(), random, transfer_msg.clone()).unwrap_err();


        // owner can
        let success = mock_info("annie", &coins(200000, "ujuno"));
        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: "random".into(),
            token_id: token_id.clone(),
        };

        let res = execute(deps.as_mut(), mock_env(), success, transfer_msg.clone());

        assert_eq!(res.is_err(), false);
    }


    #[test]
    fn approving_revoking() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = int_mgs();
        let info = mock_info("creator", &coins(1000, "ujuno"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Mint a token
        let token_id = "melt".to_string();

        let auth_info = mock_info("annie", &coins(200000, "ujuno"));
        let msg = ExecuteMsg::RegisterName { name: token_id.clone() , years: 2 , avatar_url: None, terra_address: None, secret_address: None, crypto_org_address: None, kava_address: None, persistence_address: None, starname_address: None, website: None, email: None, twitter: None, telegram: None, discord: None, instagram: None, reddit: None};
        let _res1 = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: "carl".to_string(),
            token_id: token_id.clone(),
        };

        let approved = mock_info("bobby", &coins(200000, "ujuno"));
        let res = execute(deps.as_mut(), mock_env(), approved, transfer_msg);
        println!("{:?}", res);

        // Give random transferring power
        let approve_msg = ExecuteMsg::Approve {
            spender: "bobby".into(),
            token_id: token_id.clone(),
            expires: None,
        };

        let owner = mock_info("annie", &coins(200000, "ujuno"));

        let res = execute(deps.as_mut(), mock_env(), owner, approve_msg);
        
        assert_eq!(res.is_err(), false);


        let transfer_msg = ExecuteMsg::TransferNft {
            recipient: "carl".to_string(),
            token_id: token_id.clone(),
        };

        let approved = mock_info("bobby", &coins(200000, "ujuno"));
        let res = execute(deps.as_mut(), mock_env(), approved, transfer_msg);

        println!("{:?}", res);

    }

}
