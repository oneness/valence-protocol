use cosmwasm_schema::write_api;

use valence_osmosis_gamm_withdrawer::msg::{
    FunctionMsgs, QueryMsg, ServiceConfig, ServiceConfigUpdate,
};
use valence_service_utils::msg::{ExecuteMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<ServiceConfig>,
        execute: ExecuteMsg<FunctionMsgs, ServiceConfigUpdate>,
        query: QueryMsg,
    }
}