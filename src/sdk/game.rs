use super::local_player::LocalPlayer;
use super::offsets;
use crate::sdk::player::Player;

use log::*;
use memlib::memory::{get_module, read_memory, set_global_handle, Address, Handle, Module};

// The context passed into the hack loop containing all of our
pub struct GameContext {
    pub client_module: Module,
    pub engine_module: Module,
    client_base: Address,
    engine_base: Address,
}

impl GameContext {
    pub fn new(handle: Handle) -> Self {
        // Set the global handle so we don't have to store it in the context
        set_global_handle(handle);

        let client_module = get_module("client.dll").unwrap();
        let engine_module = get_module("engine.dll").unwrap();

        let client_base = client_module.base_address;
        let engine_base = engine_module.base_address;

        Self {
            client_module,
            engine_module,
            client_base,
            engine_base,
        }
    }

    pub fn is_in_game(&self) -> bool {
        read_memory::<i32>(self.client_state_address() + offsets::signatures::dwClientState_State)
            == 6
    }

    pub fn get_local_player(&self) -> Option<LocalPlayer> {
        if !self.is_in_game() {
            return None;
        }
        LocalPlayer::new(read_memory(
            self.client_base + offsets::signatures::dwLocalPlayer,
        ))
    }

    pub fn get_entity(&self, index: u32) -> Option<Player> {
        let player_base: Address =
            read_memory(self.entity_list_address() + (index * 0x10) as Address);

        if player_base == 0 {
            return None;
        }

        trace!("Found player with base 0x{:X}", player_base);
        Player::new(player_base)
    }

    pub fn get_player_list(&self) -> Vec<Player> {
        let mut entities: Vec<Player> = Vec::new();

        for i in 0..128 {
            let player_opt = self.get_entity(i);
            if player_opt.is_none() {
                continue;
            }
            entities.push(player_opt.unwrap());
        }

        entities
    }
}

// Pointers
impl GameContext {
    fn client_state_address(&self) -> Address {
        let addr = read_memory(self.engine_base + offsets::signatures::dwClientState);
        trace!("Found client state: 0x{:X}", addr);
        addr
    }

    fn entity_list_address(&self) -> Address {
        self.client_base + offsets::signatures::dwEntityList
    }
}
