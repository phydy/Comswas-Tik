use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Game, VectorVeiw};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    InstantiateGame { other_player: String },
    PlayGame { game_id: u32, cell: (usize, usize) },
}

#[derive(QueryResponses, Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    #[returns(GameInfor)]
    GetGameInfor { game_id: u32 },
    #[returns(u32)]
    GameCount {},
    #[returns(VectorVeiw)]
    EmptyCells { game_id: u32 },
    #[returns(u8)]
    NextPlayer { game_id: u32 },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GameInfor {
    pub game_info: Game,
}
