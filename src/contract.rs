#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult, Uint128,
};
//use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MessagesResponse, QueryMsg};
use crate::state::{Message, CURRENT_ID, MESSAGES};

// version info for migration info
//const CONTRACT_NAME: &str = "crates.io:messages";
//const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // CURRENT_ID counter starts at 0
    CURRENT_ID.save(deps.storage, &Uint128::zero().u128())?;
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
        ExecuteMsg::AddMessage { topic, message } => add_message(deps, info, topic, message),
    }
}

pub fn add_message(
    deps: DepsMut,
    info: MessageInfo,
    topic: String,
    message: String,
) -> Result<Response, ContractError> {
    // Load the current id
    let mut id = CURRENT_ID.load(deps.storage)?;

    // Create message with id and function parameters
    let message = Message {
        id: Uint128::from(id),
        owner: info.sender,
        topic,
        message,
    };

    id = id.checked_add(1).unwrap();

    // Update message and updated id
    MESSAGES.save(deps.storage, id, &message)?;
    CURRENT_ID.save(deps.storage, &id)?;
    Ok(Response::new()
        .add_attribute("action", "add_message")
        .add_attribute("message id", message.id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCurrentId {} => to_binary(&query_current_id(deps)?),
        QueryMsg::GetAllMessage {} => to_binary(&query_all_messages(deps)?),
        QueryMsg::GetMessagesByAddr { address } => {
            to_binary(&query_messages_by_addr(deps, address)?)
        }
        QueryMsg::GetMessagesByTopic { topic } => to_binary(&query_messages_by_topic(deps, topic)?),
        QueryMsg::GetMessagesById { id } => to_binary(&query_messages_by_id(deps, id)?),
    }
}

fn query_current_id(deps: Deps) -> StdResult<Uint128> {
    let id = CURRENT_ID.load(deps.storage)?;
    Ok(Uint128::from(id))
}

fn query_all_messages(deps: Deps) -> StdResult<MessagesResponse> {
    // Query all  Messages entries (u128, Message), map only Messages, no filter
    let messages = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|map_data| map_data.unwrap().1)
        .collect();
    Ok(MessagesResponse { messages: messages })
}

fn query_messages_by_addr(deps: Deps, address: String) -> StdResult<MessagesResponse> {
    // Query all  Messages entries (u128, Message), map only Messages, filter messages whose owner is address
    let messages = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|map_entry| map_entry.unwrap().1)
        .filter(|message| message.owner == address)
        .collect();
    Ok(MessagesResponse { messages: messages })
}

fn query_messages_by_topic(deps: Deps, topic: String) -> StdResult<MessagesResponse> {
    // Query all  Messages entries (u128, Message), map only Messages, filter messages with topic parameter
    let messages = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|map_entry| map_entry.unwrap().1)
        .filter(|message| message.topic == topic)
        .collect();

    Ok(MessagesResponse { messages: messages })
}

fn query_messages_by_id(deps: Deps, id: Uint128) -> StdResult<MessagesResponse> {
    // Query all  Messages entries (u128, Message), map only Messages, filter messages with id parameter
    let messages = MESSAGES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|map_entry| map_entry.unwrap().1)
        .filter(|message| message.id == id)
        .collect();

    Ok(MessagesResponse { messages: messages })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    const SENDER: &str = "sender_address";

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg {};
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentId {}).unwrap();
        let value: Uint128 = from_binary(&res).unwrap();
        assert_eq!(Uint128::zero(), value);
    }

    #[test]
    fn _add_message() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

    #[test]
    fn _query_all_messages() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

    #[test]
    fn _query_messages_by_owner() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

    #[test]
    fn _query_messages_by_id() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

    #[test]
    fn _query_messages_by_topic() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }
}
