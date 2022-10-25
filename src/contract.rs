#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GameInfor, InstantiateMsg, QueryMsg};
use crate::state::{
    Board, Cell, Choice, Game, GameCount, GameStatus, VectorVeiw, Winner, WinningRow, GAMES,
    GAME_COUNT,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tic-tac-toe";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let game_count = GameCount { current_count: 1 };
    GAME_COUNT.save(deps.storage, &game_count)?;
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
        ExecuteMsg::InstantiateGame { other_player } => instantiate_game(deps, info, other_player),
        ExecuteMsg::PlayGame { game_id, cell } => execute_play_game(deps, info, game_id, cell),
    }
}

pub fn execute_play_game(
    deps: DepsMut,
    info: MessageInfo,
    game_id: u32,
    cell: (usize, usize),
) -> Result<Response, ContractError> {
    let mut game = GAMES.load(deps.storage, game_id)?;
    if game.status != GameStatus::PROGRESSING {
        return Err(ContractError::GameEndedOrUnaccepted {});
    }
    let player_address = info.sender.to_string();
    let player_choice = game.determine_player_choice(&player_address).unwrap();
    let (row, column) = cell;
    let no_choice = Cell {
        choice: Choice::None,
    };
    if row == 0 && game.board.column_0.get(column).unwrap() == &no_choice {
        game.board.column_0.insert(
            column,
            Cell {
                choice: player_choice,
            },
        );
    } else if row == 1 && game.board.column_1.get(column).unwrap() == &no_choice {
        game.board.column_1.insert(
            column,
            Cell {
                choice: player_choice,
            },
        );
    } else if row == 2 && game.board.column_2.get(column).unwrap() == &no_choice {
        game.board.column_1.insert(
            column,
            Cell {
                choice: player_choice,
            },
        );
    }
    let losing = WinningRow {
        roww: vec![(0, 0), (0, 0), (0, 0)],
    };

    for i in 1..9 {
        let player_selection = game.determine_player_choice(&player_address).unwrap();
        let cell = Cell {
            choice: player_selection,
        };
        if game.board.check_winning_row(i, cell) != losing {
            if game.player_turn == 1 {
                game.game_result = Winner::PLAYERONE
            } else {
                game.game_result = Winner::PLAYERTWO
            }
            game.status = GameStatus::ENDED;
        }
    }
    GAMES.save(deps.storage, game_id, &game)?;
    let res = Response::new().add_attribute("action", "new_game");
    Ok(res)
}

/**
 * start a new game giving your oponetnts address
 */
pub fn instantiate_game(
    deps: DepsMut,
    info: MessageInfo,
    second: String,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&second)?;
    let mut users: Vec<String> = Vec::new();
    let column1 = vec![
        Cell {
            choice: Choice::None,
        },
        Cell {
            choice: Choice::None,
        },
        Cell {
            choice: Choice::None,
        },
    ];
    let column2 = vec![
        Cell {
            choice: Choice::None,
        },
        Cell {
            choice: Choice::None,
        },
        Cell {
            choice: Choice::None,
        },
    ];
    let column3 = vec![
        Cell {
            choice: Choice::None,
        },
        Cell {
            choice: Choice::None,
        },
        Cell {
            choice: Choice::None,
        },
    ];
    users.push(info.sender.into_string());
    users.push(second);
    let game_local = Game {
        status: GameStatus::INSTANTIATED,
        player_turn: 0,
        players: users,
        board: Board {
            column_0: column1,
            column_1: column2,
            column_2: column3,
        },
        game_result: Winner::NONE,
    };
    let mut game_count = GAME_COUNT.load(deps.storage)?;
    let count = game_count.current_count;
    game_count.current_count += 1;
    GAMES.save(deps.storage, count, &game_local)?;
    GAME_COUNT.save(deps.storage, &game_count)?;
    let res = Response::new()
        .add_attribute("action", "new_game")
        .add_attribute("game_id", &game_count.current_count.to_string());
    Ok(res)
}

/**
 * accept game invitation
 */
pub fn accept_game_invite(
    deps: DepsMut,
    msg: MessageInfo,
    game_id: u32,
) -> Result<Response, ContractError> {
    let game: Game = GAMES.load(deps.storage, game_id)?;
    let second_player = game.players.get(1).unwrap();
    if &msg.sender == second_player {
        let game_to_save = Game {
            status: GameStatus::PROGRESSING,
            player_turn: 1,
            players: game.players,
            board: Board {
                column_0: game.board.column_0,
                column_1: game.board.column_1,
                column_2: game.board.column_2,
            },
            game_result: Winner::NONE,
        };
        GAMES.save(deps.storage, game_id, &game_to_save)?;
    }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGameInfor { game_id } => to_binary(&querry_game(deps, game_id)?),
        QueryMsg::GameCount {} => to_binary(&querry_game_count(deps)?),
        QueryMsg::EmptyCells { game_id } => to_binary(&querry_free_cells(deps, game_id)?),
        QueryMsg::NextPlayer { game_id } => to_binary(&querry_current_player(deps, game_id)?),
    }
}

pub fn querry_current_player(deps: Deps, game_id: u32) -> StdResult<u8> {
    let game: Game = GAMES.load(deps.storage, game_id)?;
    Ok(game.player_turn)
}

/**
 * querry the contract with a game Id to see game information
 */
pub fn querry_game(deps: Deps, game_id: u32) -> StdResult<GameInfor> {
    let game: Game = GAMES.load(deps.storage, game_id)?;
    Ok(GameInfor { game_info: game })
}

pub fn querry_game_count(deps: Deps) -> StdResult<u32> {
    let current_count = GAME_COUNT.load(deps.storage)?;
    Ok(current_count.current_count)
}

pub fn querry_free_cells(deps: Deps, game_id: u32) -> StdResult<VectorVeiw> {
    let game = GAMES.load(deps.storage, game_id)?;
    let res = game.free_cels();
    Ok(res)
}

#[cfg(test)]
mod tests {}
