use crate::ships::Position;

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub played_positions: Vec<Position>,
    pub board: [[char; 10]; 10],
    // pub ships:
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

    pub fn new() -> Player {
        return Player {
            played_positions: vec![],
            board: [[' '; 10]; 10],
        };
    }
}
