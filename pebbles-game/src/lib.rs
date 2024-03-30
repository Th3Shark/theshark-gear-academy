#![no_std]
extern crate pebbles_game_io;
extern crate gstd;

use pebbles_game_io::*;
use gstd::msg;
use gstd::exec;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    let pebbles_init: PebblesInit = msg::load().expect("Failed to load PebblesInit");

    let first_player: Player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    if let Player::Program = first_player {

    }

    let new_game_state = GameState {
        difficulty: pebbles_init.difficulty,
        first_player: first_player,
        max_pebbles_per_turn: pebbles_init.max_pebbles_per_turn,
        pebbles_count: pebbles_init.pebbles_count,
        pebbles_remaining: pebbles_init.pebbles_count,
        winner: None,
    };

    unsafe {
        PEBBLES_GAME = Some(new_game_state);
    }
}

#[no_mangle]
extern "C" fn handle() {
    // Load the received PebblesAction
    let pebbles_action: PebblesAction = msg::load().expect("Failed to load PebblesAction");

    // Load the game state
    let game_state = unsafe { PEBBLES_GAME.clone() }.expect("Game state not initialized");


    match pebbles_action {
        // If the action is a user's turn
        PebblesAction::Turn(turn) => {
            // Check if the user's turn is valid
            if turn > 0 {
                // Process the user's turn

                // Check if the user wins after the turn
                let user_wins = check_user_win_condition(&game_state);

                // Send a message to the user with the corresponding event
                let event = if user_wins {
                    PebblesEvent::Won(Player::User)
                } else {
                    PebblesEvent::CounterTurn(turn)
                };
                msg::reply(event, 0).expect("Failed to reply with PebblesEvent");
            } else {
                // If the user's turn is invalid, send an error message
                msg::reply(PebblesEvent::InvalidMove, 0).expect("Failed to reply with PebblesEvent");
            }
        }
        // If the action is to give up
        PebblesAction::GiveUp => {
            // Process the user's give up
            // (Implement the logic for processing the give up here)

            // Check if the program wins after the user gives up
            let program_wins = check_program_win_condition(&game_state); // Implement the program win condition check function

            // Send a message to the user with the corresponding event
            let event = if program_wins {
                PebblesEvent::Won(Player::Program)
            } else {
                PebblesEvent::GameEnded // Event indicating that the game ended due to user's give up
            };
            msg::reply(event, 0).expect("Failed to reply with PebblesEvent");
        }
        // If the action is to restart the game
        PebblesAction::Restart { difficulty: _, pebbles_count: _, max_pebbles_per_turn: _ } => {
            // Restart the game with the specified parameters
            // (Implement the logic for restarting the game here)
            // Send a confirmation message to the user
            msg::reply(PebblesEvent::GameRestarted, 0).expect("Failed to reply with PebblesEvent");
        }
    }
}

//  check if the user wins after their turn
fn check_user_win_condition(game_state: &GameState) -> bool {
    // Check if there are no more pebbles in the game
    game_state.pebbles_remaining == 0
}

// check if the program wins after the user gives up
fn check_program_win_condition(game_state: &GameState) -> bool {
    // Check if the user gives up and the program is next to play
    if let Some(Player::Program) = game_state.winner {
        true
    } else {
        false
    }
}

#[no_mangle]
extern "C" fn state() {
    if let Some(game_state) = unsafe { PEBBLES_GAME.clone() } {
        msg::reply(game_state, 0).expect("Failed to reply with GameState");
    } else {
        panic!("Game state not initialized");
    }
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}
