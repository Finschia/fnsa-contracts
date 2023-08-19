use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
}

#[cw_serde]
pub enum QueryMsg {
    CurrentPlan {},
    AppliedPlan {
        name: String,
    },
    ModuleVersions {
        module_name: String,
    },
}
