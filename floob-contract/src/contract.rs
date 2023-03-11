#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, OverflowError, Response, StdError,
    StdResult, Storage,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ThreadElem, Thread, THREAD_ELEM, THREAD_ELEM_COUNT, THREADS, THREAD_COUNT};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:floob-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateThread { title, description } => {
            let id = advance_posts_count(deps.storage)?;
            THREADS.save(
                deps.storage,
                id,
                &Thread {
                    title,
                    description,
                    author: info.sender,
                },
            )?;

            Ok(Response::default())
        }
        ExecuteMsg::CreateThreadElem { thread_id, content } => {
            let subthread_id = advance_subthread_count(deps.storage, thread_id)?;
            THREAD_ELEM.save(
                deps.storage,
                (thread_id, subthread_id),
                &ThreadElem {
                    content,
                    author: info.sender,
                },
            )?;
            Ok(Response::default())
        }
    }
}

/**
 * TODO: cw-paginate might be a good idea here
 */
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Thread { id } => {
            let post = THREADS.load(deps.storage, id)?;
            Ok(to_binary(&post)?)
        }
    }
}

/// MARK: Helpers

fn advance_posts_count(store: &mut dyn Storage) -> StdResult<u64> {
    let lhs = THREAD_COUNT.may_load(store)?.unwrap_or_default();
    let res = lhs.checked_add(1);
    match res {
        Some(id) => {
            THREAD_COUNT.save(store, &id)?;
            Ok(id)
        }
        None => Err(StdError::Overflow {
            source: OverflowError {
                operation: cosmwasm_std::OverflowOperation::Add,
                operand1: lhs.to_string(),
                operand2: 1.to_string(),
            },
        }),
    }
}

fn advance_subthread_count(store: &mut dyn Storage, thread_id: u64) -> StdResult<u64> {
    let lhs = THREAD_ELEM_COUNT
        .may_load(store, thread_id)?
        .unwrap_or_default();
    let res = lhs.checked_add(1);
    match res {
        Some(id) => {
            THREAD_ELEM_COUNT.save(store, thread_id, &id)?;
            Ok(id)
        }
        None => Err(StdError::Overflow {
            source: OverflowError {
                operation: cosmwasm_std::OverflowOperation::Add,
                operand1: lhs.to_string(),
                operand2: 1.to_string(),
            },
        }),
    }
}

#[cfg(test)]
mod tests {}
