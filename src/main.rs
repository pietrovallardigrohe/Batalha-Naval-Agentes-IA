use rusty_tools::*;
use ships::Position;
use std::{io::stdin, ops::Range};
pub mod ships;
pub mod utils;

#[derive(PartialEq, Eq)]
enum GameState {
    RUNNING,
    FINISHED,
}

fn main() {
    let mut board_p1 = [[0u8; 10]; 10];
    let mut board_p2 = [[0u8; 10]; 10];
    let mut game_state = GameState::RUNNING;
    ships::generate_ships(&mut board_p1);
    ships::generate_ships(&mut board_p2);

    while game_state == GameState::RUNNING {
        print_screen(board_p1, board_p2);

        println!("Digite uma escolha(padrão: \"X Y\") ou \"quit\" para sair");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        trim_newline(&mut input);
        // println!("{:?}", input);]
        match process_input(input, &mut game_state) {
            Ok(_) => {}
            Err(err) => println!("{err}"),
        }
        // clear_terminal();
    }
    // Game ending state
}

use colored::Colorize;

pub fn print_screen(board_p1: [[u8; 10]; 10], board_p2: [[u8; 10]; 10]) {
    println!("  Jogador 1:              Jogador 2:");
    print!("  ");
    for i in 0..10 {
        print!("{}{}", i.to_string().underline(), " ".underline());
    }
    print!("    ");
    for i in 0..10 {
        print!("{}{}", i.to_string().underline(), " ".underline());
    }
    print!("   ");
    println!();

    for (index, row) in board_p1.iter().enumerate() {
        print!("{}|", index);
        for column in row {
            print!("{column} ")
        }
        print!("| {}|", index);
        for column in board_p2[index] {
            print!("{column} ")
        }
        print!("|\n");
    }
}

fn process_input(input: String, game_state: &mut GameState) -> Result<Position, &str> {
    if input.to_lowercase().as_str() == "quit" {
        *game_state = GameState::FINISHED;
    }

    if input.len() == 3 {
        let mut input_iter = input.chars();
        let Some(x) = input_iter
            .next()
            .and_then(|char| char.to_digit(10))
            .and_then(|digit| digit.try_into().ok())
        else {
            return Err("Invalid input");
        };

        if input_iter.next().expect("Invalid input") != ' ' {
            return Err("Invalid input");
        }

        let Some(y) = input_iter
            .next()
            .and_then(|char| char.to_digit(10))
            .and_then(|digit| digit.try_into().ok())
        else {
            return Err("Invalid input");
        };

        println!("DEBUG: {x} {y}");
        Ok(Position { x, y })
    } else {
        return Err("Invalid input");
    }
}
