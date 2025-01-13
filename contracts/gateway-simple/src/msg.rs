use cosmwasm_schema::cw_serde;
use secret_toolkit::utils::HandleCallback;
use secret_toolkit::utils::InitCallback;

use sdk::gateway::{GatewayExecuteMsg, GatewayQueryMsg};

use crate::state::BLOCK_SIZE;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum InnerMethods {
    StoreSecret { text: String },
    Instantiate { code_id: u64, code_hash: String },
}

#[cw_serde]
pub enum InnerQueries {
    GetSecret {},
    Test {},
}

pub type ExecuteMsg = GatewayExecuteMsg<InnerMethods>;
pub type QueryMsg = GatewayQueryMsg<InnerQueries>;

#[cw_serde]
pub struct InstantiateCountMsg {
    pub count: u64,
}

impl InitCallback for InnerMethods {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}

impl HandleCallback for InnerMethods {
    const BLOCK_SIZE: usize = BLOCK_SIZE;
}
