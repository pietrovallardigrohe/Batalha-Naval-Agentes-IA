use ai::AiPlayer;
use rusty_tools::*;
use ships::Position;
use std::{io::stdin, ops::Range};
pub mod ai;
pub mod player;
pub mod ships;
pub mod utils;
use player::Player;

#[derive(PartialEq, Eq)]
enum ExecutionState {
    RUNNING,
    FINISHED,
}

static mut DEBUG_STRING: String = String::new();

fn main() {
    // let mut board_p1 = [[' 'u8; 10]; 10];
    // let mut board_p2 = [[' '; 10]; 10];
    let mut player_human = Player::new();
    let mut player_ai = AiPlayer::new(player_human.board);
    let mut game_state = ExecutionState::RUNNING;

    let mut turns: usize = 0;
    ships::generate_ships(&mut player_human);
    ships::generate_ships(&mut player_ai.player);

    while game_state == ExecutionState::RUNNING {
        clear_terminal();
        print_screen(&player_human, &player_ai.player);
        unsafe {
            println!("{DEBUG_STRING}");
            DEBUG_STRING = String::new();
        }

        if turns % 2 == 0 {
            println!("\nDigite uma escolha(padrão: \"X Y\") ou \"quit\" para sair");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            trim_newline(&mut input);
            if input.to_lowercase().as_str() == "quit" {
                game_state = ExecutionState::FINISHED;
            }
            match process_input(input) {
                Ok(position) => {
                    if player_human.shoot(position, &mut player_ai.player) {
                        turns += 1;
                        player_ai.human_board = player_human.board;
                    }
                }
                Err(err) => {
                    unsafe { DEBUG_STRING = String::from(err) };
                    ()
                }
            }
        } else {
            let position = player_ai.decide();
            unsafe {
                DEBUG_STRING.push_str(
                    format!(
                        "{position:?}\nAttempt: {}\nAttack Mode: {}",
                        player_ai.attempt, player_ai.attack_mode
                    )
                    .as_str(),
                )
            }

            if player_ai.player.shoot(position, &mut player_human) {
                turns += 1;
            }
        }
        // println!("{:?}", input);]
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

        unsafe { DEBUG_STRING.push_str(format!("\nDEBUG: {x} {y}").as_str()) };
        Ok(Position { x, y })
    } else {
        return Err("Invalid input");
    }
}
