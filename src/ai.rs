use crate::{player::Player, ships::Position};

pub struct AiPlayer {
    pub player: Player,
    pub human_board: [[char; 10]; 10],
    pub attempt: usize,
    pub attack_mode: bool,
    pub saved_index: Position,
}

impl AiPlayer {
    pub fn new(human_board: [[char; 10]; 10]) -> AiPlayer {
        return AiPlayer {
            player: Player::new(),
            human_board,
            attempt: 0,
            attack_mode: false,
            saved_index: Position { x: 0, y: 0 },
        };
    }

    pub fn decide(&mut self) -> Position {
        let last_played = match self.player.played_positions.last() {
            // BUGADO
            Some(pos) => *pos,
            None => {
                if self.attack_mode {
                    self.saved_index
                } else {
                    return Position::get_random_position();
                }
            }
        };

        if self.human_board[last_played.x][last_played.y] == 'x' || self.attack_mode {
            self.attack_mode = true;
            match self.attempt {
                0 => {
                    self.attempt += 1;
                    return Position {
                        x: last_played.x - 1,
                        y: last_played.y,
                    };
                }
                1 => {
                    self.attempt += 1;
                    return Position {
                        x: last_played.x,
                        y: last_played.y + 1,
                    };
                }
                2 => {
                    self.attempt += 1;
                    return Position {
                        x: last_played.x + 1,
                        y: last_played.y,
                    };
                }
                3 => {
                    self.attempt += 1;
                    return Position {
                        x: last_played.x,
                        y: last_played.y - 1,
                    };
                }
                _ => {
                    self.attack_mode = false;
                    self.attempt = 0;
                    return Position::get_random_position();
                }
            }
        }
        return Position::get_random_position();
    }
}
