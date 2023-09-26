use rusty_tools::*;
use ships::Position;
use std::{io::stdin, ops::Range};
pub mod player;
pub mod ships;
pub mod utils;
use player::Player;

#[derive(PartialEq, Eq)]
enum ExecutionState {
    RUNNING,
    FINISHED,
}

fn main() {
    // let mut board_p1 = [[' 'u8; 10]; 10];
    // let mut board_p2 = [[' '; 10]; 10];
    let mut player_human = Player::new();
    let mut player_ai = Player::new();
    let mut game_state = ExecutionState::RUNNING;

    let mut turns: usize = 0;

    ships::generate_ships(&mut player_human);
    ships::generate_ships(&mut player_ai);

    while game_state == ExecutionState::RUNNING {
        print_screen(&player_human, &player_ai);

        println!("Digite uma escolha(padrão: \"X Y\") ou \"quit\" para sair");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        trim_newline(&mut input);
        if input.to_lowercase().as_str() == "quit" {
            game_state = ExecutionState::FINISHED;
        }
        // println!("{:?}", input);]
        match process_input(input) {
            Ok(position) => {
                if turns % 2 == 0 {
                    player_human.shoot(position, &mut player_ai);
                } else {
                    player_ai.shoot(position, &mut player_human);
                }
                turns += 1;
            }
            Err(err) => {
                println!("{err}");
                ()
            }
        }
        // clear_terminal();
    }
    // Game ending state
}

use colored::Colorize;

pub fn print_screen(player_1: &Player, player_2: &Player) {
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

    for (index, row) in player_1.board.iter().enumerate() {
        print!("{}|", index);
        for column in row {
            print!("{column} ")
        }
        print!("| {}|", index);
        for column in player_2.board[index] {
            print!("{column} ")
        }
        print!("|\n");
    }
}

fn process_input<'a>(input: String) -> Result<Position, &'a str> {
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
