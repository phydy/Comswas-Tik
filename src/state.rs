use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::u8;

use crate::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub enum GameStatus {
    PROGRESSING,
    ENDED,
    INSTANTIATED,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Winner {
    PLAYERONE,
    PLAYERTWO,
    DRAW,
    NONE,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct VectorVeiw {
    pub column_0: Vec<(i32, i32)>,
    pub column_1: Vec<(i32, i32)>,
    pub column_2: Vec<(i32, i32)>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Game {
    /**
     * the status of the game: ie, if active or ended
     */
    pub status: GameStatus,
    /**
     * the player to play next, 0 for first address in the vector below
     * 1 for the address 2cond address
     */
    pub player_turn: u8,
    /**
     * addresses allowed to play on this game: can oly be two
     */
    pub players: Vec<String>,
    /**
     * the game board
     */
    pub board: Board,

    /**
     * showning the winner
     */
    pub game_result: Winner,
}

impl Game {
    pub fn determine_player_choice(&self, sender: &String) -> Result<Choice, ContractError> {
        let players = &self.players;
        let initiator = players.get(0).unwrap();
        let invitee = players.get(1).unwrap();

        let full_string = initiator.to_string() + &invitee;
        let mut hasher = Sha256::new();
        hasher.update(full_string);
        let result = hasher.finalize();
        if result.starts_with(&[0]) {
            if &sender == &initiator {
                Ok(Choice::O)
            } else {
                Ok(Choice::X)
            }
        } else {
            if &sender == &initiator {
                Ok(Choice::X)
            } else {
                Ok(Choice::O)
            }
        }
    }

    /**
     * check if an address is allowed to play a game
     */
    pub fn is_allowed_player(&self, checked_address: String) -> Result<u8, ContractError> {
        let sending_address = &self.players;
        if sending_address.contains(&checked_address) {
            Ok(0)
        } else {
            Ok(1)
        }
    }

    /**
     * get the status of the game
     */
    pub fn status(&self) -> Result<&GameStatus, ContractError> {
        Ok(&self.status)
    }

    /**
     * returns the free cells in each column of the board
     */

    pub fn free_cels(&self) -> VectorVeiw {
        let first_column = &self.board.column_0;
        let second_column = &self.board.column_1;
        let third_column = &self.board.column_2;
        let mut free_cell1: Vec<(i32, i32)> = Vec::new();
        let mut free_cell2: Vec<(i32, i32)> = Vec::new();
        let mut free_cell3: Vec<(i32, i32)> = Vec::new();

        for i in 0..2 {
            //let current_choice = first_column.get(i);
            if first_column.get(i)
                == Some(&Cell {
                    choice: Choice::None,
                })
            {
                free_cell1.push((0, i.try_into().unwrap()));
            }
            if second_column.get(i)
                == Some(&Cell {
                    choice: Choice::None,
                })
            {
                let to_be_pushed = (1, i.try_into().unwrap());

                free_cell2.push(to_be_pushed);
            }
            if third_column.get(i)
                == Some(&Cell {
                    choice: Choice::None,
                })
            {
                let to_be_pushed = (2, i.try_into().unwrap());
                free_cell3.push(to_be_pushed);
            }
        }
        let res = VectorVeiw {
            column_0: free_cell1,
            column_1: free_cell2,
            column_2: free_cell3,
        };

        res
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Board {
    pub column_0: Vec<Cell>,
    pub column_1: Vec<Cell>,
    pub column_2: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct WinningRow {
    pub roww: Vec<(i32, i32)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Row {
    pub row_selected: u8,
}

impl Board {
    pub fn check_winning_row(&mut self, row: u8, player_choice: Cell) -> WinningRow {
        let first_row = vec![(0, 0), (0, 1), (0, 2)];
        let second_row = vec![(1, 0), (1, 1), (1, 2)];
        let third_row = vec![(2, 0), (2, 1), (2, 2)];
        let first_column = vec![(2, 0), (1, 0), (0, 0)];
        let second_column = vec![(2, 1), (1, 1), (0, 1)];
        let third_column = vec![(2, 2), (1, 2), (0, 2)];
        let diagonal_one = vec![(0, 2), (1, 1), (2, 0)];
        let diagonal_two = vec![(0, 0), (1, 1), (2, 2)];
        let board_column_0 = &self.column_0;
        let board_column_1 = &self.column_1;
        let board_column_2 = &self.column_2;
        let losing = WinningRow {
            roww: vec![(0, 0), (0, 0), (0, 0)],
        };

        if row == 1 {
            let cell1 = board_column_0.get(0).unwrap();
            let cell2 = board_column_0.get(1).unwrap();
            let cell3 = board_column_0.get(2).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: first_row };
                winning
            } else {
                losing
            }
        } else if row == 2 {
            let cell1 = board_column_1.get(0).unwrap();
            let cell2 = board_column_1.get(1).unwrap();
            let cell3 = board_column_1.get(2).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: second_row };
                winning
            } else {
                losing
            }
        } else if row == 3 {
            let cell1 = board_column_2.get(0).unwrap();
            let cell2 = board_column_2.get(1).unwrap();
            let cell3 = board_column_2.get(2).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: third_row };
                winning
            } else {
                losing
            }
        } else if row == 4 {
            let cell1 = board_column_2.get(0).unwrap();
            let cell2 = board_column_1.get(0).unwrap();
            let cell3 = board_column_0.get(0).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: first_column };
                winning
            } else {
                losing
            }
        } else if row == 5 {
            let cell1 = board_column_2.get(1).unwrap();
            let cell2 = board_column_1.get(1).unwrap();
            let cell3 = board_column_0.get(1).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow {
                    roww: second_column,
                };
                winning
            } else {
                losing
            }
        } else if row == 6 {
            let cell1 = board_column_2.get(2).unwrap();
            let cell2 = board_column_1.get(2).unwrap();
            let cell3 = board_column_0.get(2).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: third_column };
                winning
            } else {
                losing
            }
        } else if row == 7 {
            let cell1 = board_column_0.get(2).unwrap();
            let cell2 = board_column_1.get(1).unwrap();
            let cell3 = board_column_2.get(0).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: diagonal_one };
                winning
            } else {
                losing
            }
        } else if row == 8 {
            let cell1 = board_column_0.get(0).unwrap();
            let cell2 = board_column_1.get(1).unwrap();
            let cell3 = board_column_2.get(2).unwrap();
            if cell1 == cell2 && cell2 == cell3 && cell3 == &player_choice {
                let winning = WinningRow { roww: diagonal_two };
                winning
            } else {
                losing
            }
        } else {
            losing
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cell {
    pub choice: Choice,
}

impl Cell {
    pub fn make_choice(&mut self, selection: Choice) -> Result<&Choice, ContractError> {
        if &self.choice == &Choice::None {
            self.choice = selection;
        }
        Ok(&self.choice)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Choice {
    X,
    O,
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct GameCount {
    pub current_count: u32,
}
pub const GAME_COUNT: Item<GameCount> = Item::new("game_count");
pub const GAMES: Map<u32, Game> = Map::new("games");
