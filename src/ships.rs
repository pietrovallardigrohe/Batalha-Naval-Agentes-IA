use crate::{player::Player, utils};
use core::fmt;
use std::{error::Error, fmt::Display};
use utils::random_point;

// type Result<T> = std::result::Result<T, SpaceOccupied>;
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn generate_ships(player: &mut Player) {
    let board = &mut player.board;
    let mut iteration = 0;
    while generate_l(board).is_err() && iteration < 100 {
        iteration += 1;
    }

    let mut iteration = 0;
    while generate_z(board).is_err() && iteration < 100 {
        iteration += 1;
    }

    let mut iteration = 0;
    while generate_t(board).is_err() && iteration < 100 {
        iteration += 1;
    }

    for _ in 0..2 {
        let mut iteration = 0;
        while generate_duo(board).is_err() && iteration < 100 {
            iteration += 1;
        }
    }
}

// Generate L shape ship
fn generate_l(board: &mut [[char; 10]; 10]) -> Result<[usize; 2], ()> {
    let [x, y] = random_point(7, 8);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1'
        || board[x + 1][y] == '1'
        || board[x + 2][y] == '1'
        || board[x + 2][y + 1] == '1'
    {
        println!("ERROR: Initial position: X = {x}, Y = {y}");
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

    Ok([x, y])
}

fn generate_z(board: &mut [[char; 10]; 10]) -> Result<[usize; 2], ()> {
    let [x, y] = random_point(7, 8);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1'
        || board[x][y + 1] == '1'
        || board[x + 1][y + 1] == '1'
        || board[x + 2][y + 1] == '1'
    {
        println!("ERROR: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    board[x][y] = '1';
    board[x + 1][y] = '1';
    board[x + 1][y + 1] = '1';
    board[x + 2][y + 1] = '1';
    // println!("DEBUG: {:?}", board);
    Ok([x, y])
}

fn generate_t(board: &mut [[char; 10]; 10]) -> Result<[usize; 2], ()> {
    let [x, y] = random_point(7, 7);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1'
        || board[x][y + 1] == '1'
        || board[x][y + 2] == '1'
        || board[x + 1][y + 1] == '1'
        || board[x + 2][y + 1] == '1'
    {
        println!("ERROR: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    board[x][y] = '1';
    board[x][y + 1] = '1';
    board[x][y + 2] = '1';
    board[x + 1][y + 1] = '1';
    board[x + 2][y + 1] = '1';
    // println!("DEBUG: {:?}", board);
    Ok([x, y])
}

fn generate_duo(board: &mut [[char; 10]; 10]) -> Result<[usize; 2], ()> {
    let [x, y] = random_point(9, 8);
    // println!("X: {x}, Y: {y}");

    if board[x][y] == '1' || board[x][y + 1] == '1' {
        println!("ERROR: Initial position: X = {x}, Y = {y}");
        return Err(());
    }

    board[x][y] = '1';
    board[x][y + 1] = '1';
    // println!("DEBUG: {:?}", board);
    Ok([x, y])
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
        for _ in 0..100 {
            let mut player = Player::new();
            generate_ships(&mut player);
            crate::print_screen(&player, &player);
        }
    }
}
