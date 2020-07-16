use memlib::hacks::interfaces;
use memlib::math::{Rotation3, Vector3};
use memlib::memory::{read_memory, Address};

use super::offsets;
use crate::sdk::player::Player;

pub struct LocalPlayer {
    base: Address,
    player: Player, // Store a normal player object so we can call its intrinsic functions
}

impl LocalPlayer {
    pub fn new(base_address: Address) -> Option<LocalPlayer> {
        let player = Player::new(base_address)?;
        Some(LocalPlayer {
            base: base_address,
            player,
        })
    }
}

impl interfaces::Player for LocalPlayer {
    fn get_origin(&self) -> Vector3 {
        self.player.get_origin()
    }

    fn get_name(&self) -> String {
        self.player.get_name()
    }

    fn get_view_angles(&self) -> Rotation3 {
        self.player.get_view_angles()
    }

    fn get_health(&self) -> i32 {
        self.player.get_health()
    }

    fn get_team_num(&self) -> i32 {
        self.player.get_team_num()
    }

    fn is_alive(&self) -> bool {
        self.player.is_alive()
    }
}
