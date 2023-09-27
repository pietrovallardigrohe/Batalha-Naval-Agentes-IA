use crate::ships::{Position, Ship};

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub played_positions: Vec<Position>,
    pub board: [[char; 10]; 10],
    pub number_ships: usize,
    pub ships_destroyed: usize,
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn shoot(&mut self, position: Position, player: &mut Player) -> bool {
        if !self.played_positions.contains(&position) {
            let board_position = &mut player.board[position.x][position.y];

            if *board_position == '1' {
                *board_position = 'x';
            } else {
                *board_position = 'o';
            }
            self.played_positions.push(position);
        } else {
            return false;
        }

        return true;
    }

    pub fn new(ships: Vec<Ship>, board: [[char; 10]; 10]) -> Player {
        return Player {
            played_positions: vec![],
            board,
            number_ships: 5,
            ships_destroyed: 0,
            ships,
        };
    }
}

impl Default for Player {
    fn default() -> Self {
        return Player {
            played_positions: vec![],
            board: [[' '; 10]; 10],
            number_ships: 5,
            ships_destroyed: 0,
            ships: vec![],
        };
    }
}
