use ai::AiPlayer;
use rusty_tools::*;
use ships::Position;
use std::io::stdin;
pub mod ai;
pub mod player;
pub mod ships;
pub mod utils;
use player::Player;

#[derive(PartialEq, Eq)]
enum ExecutionState {
    Running,
    Finished,
    Debug,
}

// static mut DEBUG_STRING: String = String::new();

fn main() {
    // let mut board_p1 = [[' 'u8; 10]; 10];
    // let mut board_p2 = [[' '; 10]; 10];
    let gen = ships::generate_ships();
    let mut player_human = Player::new(gen.0, gen.1);
    let gen = ships::generate_ships();
    let mut player_ai = AiPlayer::new(Player::new(gen.0, gen.1), player_human.board);
    let mut game_state = ExecutionState::Running;

    let mut turns: usize = 0;

    let mut print_buffer = String::new();

    print_buffer.push_str("\n\"{0..9} {0..9}\" = Shoot \ndebug = AI actions description \nhelp = Displays commands \nquit = Exit program");

    while game_state == ExecutionState::Running || game_state == ExecutionState::Debug {
        clear_terminal();
        print_screen(&player_human, &player_ai.player);
        println!("{print_buffer}");

        if turns % 2 == 0 {
            print_buffer = String::new();
            println!("\nDigite uma escolha(padrão: \"X Y\") ou \"quit\" para sair");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            trim_newline(&mut input);

            match input.to_lowercase().as_str() {
                "quit" => game_state = ExecutionState::Finished,
                "debug" => {
                    game_state = ExecutionState::Debug;
                    print_buffer.push_str("DEBUG MODE ACTIVE");
                }
                "help" => {
                    print_buffer.push_str("\n\"{0..9} {0..9}\" = Shoot \ndebug = AI actions description \nhelp = Displays commands \nquit = Exit program");
                }
                _ => match process_input(input) {
                    Ok(position) => {
                        if player_human.shoot(position, &mut player_ai.player) {
                            check_ships(&mut player_human, &mut player_ai.player);
                            turns += 1;
                            player_ai.human_board = player_human.board;
                        }
                    }
                    Err(err) => {
                        print_buffer = format!("ERROR: {err}");
                    }
                },
            }
        } else {
            let position = player_ai.decide();
            if game_state == ExecutionState::Debug {
                print_buffer.push_str(
                    format!(
                        "\nAI Behaviour: \nChoice: {position:?} \nAttempt: {} \nAttack Mode: {} \nLast position: {:?}",
                        player_ai.attempt,
                        player_ai.attack_mode,
                        player_ai
                            .player
                            .played_positions
                            .last()
                            .unwrap_or(&Position { x: 0, y: 0 })
                    )
                    .as_str(),
                );
            }

            if player_ai.player.shoot(position, &mut player_human) {
                check_ships(&mut player_ai.player, &mut player_human);
                turns += 1;
            }
        }
        // println!("{:?}", input);]

        if player_human.number_ships == 0 {
            print_screen(&player_human, &player_ai.player);
            println!("AI ganhou");
            game_state = ExecutionState::Finished;
        } else if player_ai.player.number_ships == 0 {
            print_screen(&player_human, &player_ai.player);
            println!("Humano ganhou");
            game_state = ExecutionState::Finished;
        }
    }
    // Game ending state
}

use colored::Colorize;

use crate::ships::check_ships;

pub fn print_screen(player_1: &Player, player_2: &Player) {
    println!("  Jogador 1:              Jogador AI:");
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
        println!("|");
    }

    println!(
        "Número de navios: {}     Número de navios: {}",
        player_1.number_ships, player_2.number_ships
    );
    println!(
        "Navios destruídos: {}    Navios destruídos: {}",
        player_1.ships_destroyed, player_2.ships_destroyed
    );
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

        // print_buffer.push_str(format!("\nDEBUG: {x} {y}").as_str());
        Ok(Position { x, y })
    } else {
        Err("Invalid input")
    }
}
