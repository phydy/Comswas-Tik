use cosmwasm_schema::cw_serde;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Game;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    InstantiateGame { other_player: String },
    PlayGame { game_id: u32, cell: (usize, usize) },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    GetGameInfor { game_id: u32 },
    GameCount {},
    EmptyCells { game_id: u32 },
    NextPlayer { game_id: u32 },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GameInfor {
    pub game_info: Game,
}
