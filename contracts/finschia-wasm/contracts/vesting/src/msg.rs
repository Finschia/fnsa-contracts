use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::v1beta1::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateVestingAccount {
        from_address: String,
        to_address: String,
        amount: Vec<Coin>,
        end_time: i64,
        delayed: bool,
    },
}

#[cw_serde]
pub enum QueryMsg {}
