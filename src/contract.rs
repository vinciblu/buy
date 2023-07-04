#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, Coin,
};
use cosmwasm_std::to_binary;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn buyer_key(address: &Addr) -> Vec<u8> {
    format!("buyer_{}", address).into_bytes()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.storage.set(b"owner", msg.owner.as_bytes());
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyAndStoreData {
            cart_price_usd,
            encrypted_data,
        } => try_buy_and_store_data(deps, env, info, cart_price_usd, encrypted_data),
        ExecuteMsg::WithdrawFunds { amount } => try_withdraw(deps, env, info, amount),
    }
}

// New function to handle both buy and store_encrypted_data messages
fn try_buy_and_store_data(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cart_price_usd: Uint128,
    encrypted_data: String,
) -> Result<Response, ContractError> {
    let contract_address = env.contract.address;

    // Get the conversion rate (example value of 5u128)
    let stake_per_usd = Uint128::new(1);

    // Calculate the total price of the cart in "stake"
    let cart_price_stake = cart_price_usd * stake_per_usd;

    if info.funds.len() != 1 || info.funds[0].denom != "stake" || info.funds[0].amount != cart_price_stake {
        return Err(ContractError::InvalidFunds {
            expected: cart_price_stake,
            got: info.funds.into_iter().map(|coin| coin.amount).sum(),
        });
    }

    // The funds are already transferred to the contract by the buyer when executing the transaction
    let transfer_msg = format!(
        "Transferred {} tokens from {} to {}",
        cart_price_stake, info.sender, contract_address
    );
// store the encrypted data
    let buyer_key = buyer_key(&info.sender);
    deps.storage.set(&buyer_key, encrypted_data.as_bytes());

    // Return a response with the transfer message and store_encrypted_address action
    Ok(Response::new()
        .add_attribute("action", "buy_and_store_data")
        .add_attribute("message", transfer_msg))
}

fn try_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let owner_bytes = deps
        .storage
        .get(b"owner")
        .ok_or(ContractError::NoDataStored { })?;
    let owner_str = String::from_utf8(owner_bytes).map_err(|_| ContractError::InvalidData { })?;
    let owner: Addr = Addr::unchecked(owner_str);

    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let coin = Coin {
        denom: "stake".to_string(),
        amount,
    };
    let amount_vec = vec![coin];
    let res: cosmwasm_std::CosmosMsg = cosmwasm_std::BankMsg::Send {
        to_address: owner.to_string(),
        amount: amount_vec,
    }
    .into();

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_message(res))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEncryptedData { buyer_address, sender } => {
            let owner_bytes = deps
                .storage
                .get(b"owner")
                .ok_or(StdError::not_found("NoDataStored"))?;
            let owner_str = String::from_utf8(owner_bytes).map_err(|_| {
                StdError::parse_err("InvalidData", "Cannot parse owner")
            })?;
            let owner: Addr = Addr::unchecked(owner_str);

            if sender != owner {
                return Err(StdError::generic_err("Unauthorized"));
            }

            let buyer_addr = Addr::unchecked(buyer_address);
            let buyer_key = buyer_key(&buyer_addr);
            let encrypted_data_bytes = deps
                .storage
                .get(&buyer_key)
                .ok_or(StdError::not_found("NoDataStored"))?;
            let encrypted_data = String::from_utf8(encrypted_data_bytes).map_err(|_| {
                StdError::parse_err("InvalidData", "Cannot parse encrypted_data")
            })?;
            to_binary(&encrypted_data)
        }
    }
}

    

#[cfg(test)]
mod tests {}