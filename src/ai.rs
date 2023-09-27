use crate::{player::Player, ships::Position};

pub struct AiPlayer {
    pub player: Player,
    pub human_board: [[char; 10]; 10],
    pub attempt: usize,
    pub attack_mode: bool,
    pub saved_index: Position,
}

impl AiPlayer {
    pub fn new(player: Player, human_board: [[char; 10]; 10]) -> AiPlayer {
        AiPlayer {
            player,
            human_board,
            attempt: 0,
            attack_mode: false,
            saved_index: Position { x: 0, y: 0 },
        }
    }

    pub fn decide(&mut self) -> Position {
        let last_played = match self.player.played_positions.last() {
            Some(pos) => {
                if self.attack_mode
                    && (self.human_board[pos.x][pos.y] == 'o'
                        || self.human_board[pos.x][pos.y] == 'x')
                {
                    // self.saved_index
                    self.saved_index
                } else {
                    self.attempt = 0;
                    *pos
                }
            }
            None => {
                return Position::get_random_position();
            }
        };

        if self.human_board[last_played.x][last_played.y] == 'x' || self.attack_mode {
            if !self.attack_mode {
                self.saved_index = last_played;
            }
            self.attack_mode = true;
            match self.attempt {
                0 => {
                    self.attempt += 1;
                    if last_played.x == 0
                        || self.human_board[last_played.x - 1][last_played.y] == 'x'
                        || self.human_board[last_played.x - 1][last_played.y] == 'o'
                    {
                        self.attack_mode = false;
                        return Position::get_random_position();
                    }
                    return Position {
                        x: last_played.x - 1,
                        y: last_played.y,
                    };
                }
                1 => {
                    self.attempt += 1;
                    if last_played.y == 9
                        || self.human_board[last_played.x][last_played.y + 1] == 'x'
                        || self.human_board[last_played.x][last_played.y + 1] == 'o'
                    {
                        self.attack_mode = false;
                        return Position::get_random_position();
                    }
                    return Position {
                        x: last_played.x,
                        y: last_played.y + 1,
                    };
                }
                2 => {
                    self.attempt += 1;
                    if last_played.x == 9
                        || self.human_board[last_played.x + 1][last_played.y] == 'x'
                        || self.human_board[last_played.x + 1][last_played.y] == 'o'
                    {
                        self.attack_mode = false;
                        return Position::get_random_position();
                    }
                    return Position {
                        x: last_played.x + 1,
                        y: last_played.y,
                    };
                }
                3 => {
                    self.attempt += 1;
                    if last_played.y == 0
                        || self.human_board[last_played.x][last_played.y - 1] == 'x'
                        || self.human_board[last_played.x][last_played.y - 1] == 'o'
                    {
                        self.attack_mode = false;
                        return Position::get_random_position();
                    }
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
        self.attack_mode = false;
        Position::get_random_position()
    }
}
