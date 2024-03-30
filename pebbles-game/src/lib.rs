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

    let first_player: pebbles_game_io::Player = if get_random_u32() % 2 == 0 {
        pebbles_game_io::Player::User
    } else {
        pebbles_game_io::Player::Program
    };

    if let pebbles_game_io::Player::Program = first_player {
        // Removido println! devido à falta de suporte a saída de console
    }

    let new_game_state = GameState {
        difficulty: pebbles_init.difficulty,
        first_player: first_player,
        max_pebbles_per_turn: 0,
        pebbles_count: 0,
        pebbles_remaining: 0,
        winner: None,
    };

    unsafe {
        PEBBLES_GAME = Some(new_game_state);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let pebbles_action: PebblesAction = msg::load().expect("Failed to load PebblesAction");

    // Removido println! devido à falta de suporte a saída de console

    // Removido println! devido à falta de suporte a saída de console

    // Removido println! devido à falta de suporte a saída de console
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
