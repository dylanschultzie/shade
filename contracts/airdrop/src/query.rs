use cosmwasm_std::{Api, Extern, Querier, StdError, StdResult, Storage, HumanAddr, Uint128};
use shade_protocol::airdrop::{QueryAnswer};
use crate::{state::{config_r, reward_r}};
use crate::state::claim_status_r;

pub fn config<S: Storage, A: Api, Q: Querier>
(deps: &Extern<S, A, Q>) -> StdResult<QueryAnswer> {
    Ok(QueryAnswer::Config { config: config_r(&deps.storage).load()?
    })
}

pub fn dates<S: Storage, A: Api, Q: Querier>
(deps: &Extern<S, A, Q>) -> StdResult<QueryAnswer> {
    let config = config_r(&deps.storage).load()?;
    Ok(QueryAnswer::Dates { start: config.start_date, end: config.end_date
    })
}

pub fn airdrop_amount<S: Storage, A: Api, Q: Querier>
(deps: &Extern<S, A, Q>, address: HumanAddr) -> StdResult<QueryAnswer> {
    let key = address.to_string();

    let eligible_amount = reward_r(&deps.storage).load(key.as_bytes())?.amount;

    let mut finished_tasks = vec![];
    let mut claimed = Uint128::zero();
    let mut unclaimed = Uint128::zero();

    let config = config_r(&deps.storage).load()?;
    for (i, task) in config.task_claim.iter().enumerate() {
        let state = claim_status_r(&deps.storage, i).may_load(key.as_bytes())?;

        match state {
            None => {}
            Some(task_claimed) => {
                finished_tasks.push(task.clone());
                let calc = task.percent.multiply_ratio(eligible_amount.clone(),
                                                       Uint128(100));
                match task_claimed {
                    true => claimed += calc,
                    false => unclaimed += calc
                };
            }
        };
    }

    Ok(QueryAnswer::Eligibility {
        total: eligible_amount,
        claimed,
        unclaimed,
        finished_tasks
    })
}