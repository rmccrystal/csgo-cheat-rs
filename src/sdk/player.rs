use memlib::hacks::interfaces;
use memlib::math::{Rotation3, Vector3};
use memlib::memory::{read_memory, read_string, Address};

use super::offsets::netvars;

pub struct Player {
    base: Address,
}

impl Player {
    /// Creates a new player. Returns None if the newly created player is not valid
    pub fn new(base_address: Address) -> Option<Player> {
        let player = Player { base: base_address };

        if !player.is_valid() {
            return None;
        }

        Some(player)
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}

impl interfaces::Player for Player {
    fn get_origin(&self) -> Vector3 {
        read_memory(self.base + netvars::m_vecOrigin)
    }

    fn get_name(&self) -> String {
        read_string(self.base + netvars::m_szCustomName)
    }

    fn get_view_angles(&self) -> Rotation3 {
        read_memory(self.base + netvars::m_vecViewOffset)
    }

    fn get_health(&self) -> i32 {
        read_memory(self.base + netvars::m_iHealth)
    }

    fn get_team_num(&self) -> i32 {
        read_memory(self.base + netvars::m_iTeamNum)
    }

    fn is_alive(&self) -> bool {
        self.get_health() > 0 && self.get_health() <= 2000
    }
}
