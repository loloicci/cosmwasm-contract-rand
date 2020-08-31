use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdResult, Storage,
};
use crate::msg::{QueryResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

use rand::Rng;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut rng = rand::thread_rng();
    let rand: u64 = rng.gen();

    let state = State {
        rand: rand,
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Update {} => try_update(deps, env),
    }
}

pub fn try_update<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        let mut rng = rand::thread_rng();
        let rand: u64 = rng.gen();
        state.rand = rand;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Get {} => to_binary(&query_get(deps)?),
    }
}

fn query_get<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(QueryResponse { rand: state.rand })
}
