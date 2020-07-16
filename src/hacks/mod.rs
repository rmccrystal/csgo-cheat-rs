use crate::config::Config;
use crate::sdk::GameContext;
use crate::CHEAT_TICKRATE;

use log::*;
use memlib::hacks::interfaces::Player;
use memlib::memory::{get_module, read_memory, Address, Pointer};
use memlib::util::{to_hex_string, LoopTimer};
use std::error::Error;

// The main loop of the cheat
// Returns an error if there is an error with any of the tick functions
pub fn hack_loop(ctx: GameContext) -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    let mut timer = LoopTimer::new(CHEAT_TICKRATE);

    loop {
        timer.wait();
        let player_list = ctx.get_player_list();
        dbg!(player_list.len());
        // for player in player_list {
        //     debug!("Health: {}", player.get_health());
        // }
    }

    Ok(())
}
