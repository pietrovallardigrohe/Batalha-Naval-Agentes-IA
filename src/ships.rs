use crate::{player::Player, utils};
use rand::{thread_rng, Rng};
use utils::random_point;

// type Result<T> = std::result::Result<T, SpaceOccupied>;
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn get_random_position() -> Position {
        Position {
            x: thread_rng().gen_range(0..10),
            y: thread_rng().gen_range(0..10),
        }
    }

    pub fn new(pos: [usize; 2]) -> Position {
        Position {
            x: pos[0],
            y: pos[1],
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ShipType {
    T,
    L,
    Z,
    D, // Duo
}

#[derive(Clone, PartialEq, Eq)]
pub struct Ship {
    starting_position: Position,
    is_destroyed: bool,
    ship_type: ShipType,
}

impl Ship {
    fn new(starting_position: Position, ship_type: ShipType) -> Ship {
        Ship {
            starting_position,
            is_destroyed: false,
            ship_type,
        }
    }
}

pub fn generate_ships() -> (Vec<Ship>, [[char; 10]; 10]) {
    let mut board = [[' '; 10]; 10];
    let mut ships: Vec<Ship> = vec![];

    let mut iteration = 0;
    while iteration < 100 {
        if let Ok(l) = generate_l(&mut board) {
            ships.push(l);
            break;
        } else {
            iteration += 1;
        }
    }

    iteration = 0;
    while iteration < 100 {
        if let Ok(z) = generate_z(&mut board) {
            ships.push(z);
            break;
        } else {
            iteration += 1;
        }
    }

    iteration = 0;
    while iteration < 100 {
        if let Ok(t) = generate_t(&mut board) {
            ships.push(t);
            break;
        } else {
            iteration += 1;
        }
    }
    for _ in 0..2 {
        let mut iteration = 0;
        while iteration < 100 {
            if let Ok(duo) = generate_duo(&mut board) {
                ships.push(duo);
                break;
            } else {
                iteration += 1;
            }
        }
    }
    (ships, board)
}

// Generate L shape ship
fn generate_l(board: &mut [[char; 10]; 10]) -> Result<Ship, ()> {
    let [x, y] = random_point(7, 8);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1'
        || board[x + 1][y] == '1'
        || board[x + 2][y] == '1'
        || board[x + 2][y + 1] == '1'
    {
        println!("ERROR generate_l: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    // Valid positions
    // Plotting...
    for i in 0..3 {
        let current = &mut board[x + i][y];
        *current = '1';
    }
    board[x + 2][y + 1] = '1';
    // println!("DEBUG: {:?}", board);

    // *current = 1;
    // board[x + 1][y + 2] = 1;

    Ok(Ship::new(Position::new([x, y]), ShipType::L))
}

fn generate_z(board: &mut [[char; 10]; 10]) -> Result<Ship, ()> {
    let [x, y] = random_point(7, 8);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1'
        || board[x][y + 1] == '1'
        || board[x + 1][y + 1] == '1'
        || board[x + 2][y + 1] == '1'
    {
        println!("ERROR generate_z: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    board[x][y] = '1';
    board[x + 1][y] = '1';
    board[x + 1][y + 1] = '1';
    board[x + 2][y + 1] = '1';
    // println!("DEBUG: {:?}", board);
    Ok(Ship::new(Position::new([x, y]), ShipType::Z))
}

fn generate_t(board: &mut [[char; 10]; 10]) -> Result<Ship, ()> {
    let [x, y] = random_point(7, 7);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1'
        || board[x][y + 1] == '1'
        || board[x][y + 2] == '1'
        || board[x + 1][y + 1] == '1'
        || board[x + 2][y + 1] == '1'
    {
        println!("ERROR generate_t: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    board[x][y] = '1';
    board[x][y + 1] = '1';
    board[x][y + 2] = '1';
    board[x + 1][y + 1] = '1';
    board[x + 2][y + 1] = '1';
    // println!("DEBUG: {:?}", board);
    Ok(Ship::new(Position::new([x, y]), ShipType::T))
}

fn generate_duo(board: &mut [[char; 10]; 10]) -> Result<Ship, ()> {
    let [x, y] = random_point(9, 8);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1' || board[x][y + 1] == '1' {
        println!("ERROR generate_duo: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    board[x][y] = '1';
    board[x][y + 1] = '1';
    // println!("DEBUG: {:?}", board);
    Ok(Ship::new(Position::new([x, y]), ShipType::D))
}

pub fn check_ships(player_1: &mut Player, player_2: &mut Player) {
    for ship in &mut player_2.ships {
        if !ship.is_destroyed {
            match ship.ship_type {
                ShipType::D => {
                    if player_1.played_positions.contains(&ship.starting_position)
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x,
                            y: ship.starting_position.y + 1,
                        })
                    {
                        ship.is_destroyed = true;
                        player_1.ships_destroyed += 1;
                    }
                }
                ShipType::L => {
                    if player_1.played_positions.contains(&ship.starting_position)
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 1,
                            y: ship.starting_position.y,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 2,
                            y: ship.starting_position.y,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 2,
                            y: ship.starting_position.y + 1,
                        })
                    {
                        ship.is_destroyed = true;
                        player_1.ships_destroyed += 1;
                    }
                }
                ShipType::Z => {
                    if player_1.played_positions.contains(&ship.starting_position)
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 1,
                            y: ship.starting_position.y,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 1,
                            y: ship.starting_position.y + 1,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 2,
                            y: ship.starting_position.y + 1,
                        })
                    {
                        ship.is_destroyed = true;
                        player_1.ships_destroyed += 1;
                    }
                }
                ShipType::T => {
                    if player_1.played_positions.contains(&ship.starting_position)
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x,
                            y: ship.starting_position.y + 1,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x,
                            y: ship.starting_position.y + 2,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 1,
                            y: ship.starting_position.y + 1,
                        })
                        && player_1.played_positions.contains(&Position {
                            x: ship.starting_position.x + 2,
                            y: ship.starting_position.y + 1,
                        })
                    {
                        ship.is_destroyed = true;
                        player_1.ships_destroyed += 1;
                    }
                }
            }
        }
    }
    player_2.number_ships = 5 - player_1.ships_destroyed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singles() {
        for _ in 0..10000 {
            let mut board = [[' '; 10]; 10];
            // generate_ships(&mut board);
            assert!(generate_l(&mut board).is_ok());
            let mut board = [[' '; 10]; 10];
            assert!(generate_z(&mut board).is_ok());
            let mut board = [[' '; 10]; 10];
            assert!(generate_t(&mut board).is_ok());
            let mut board = [[' '; 10]; 10];
            assert!(generate_duo(&mut board).is_ok());
        }
    }

    #[test]
    fn create_ships() {
        for _ in 0..1 {
            println!("B1");
            let (_, board) = generate_ships();
            println!("B2");
            let (_, board2) = generate_ships();

            let mut p1 = Player::default();
            p1.board = board;

            let mut p2 = Player::default();
            p2.board = board2;
            crate::print_screen(&p1, &p2);
        }
    }
}
