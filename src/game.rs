mod player;

use crate::engine::LudoEngine;
use crate::display::Display;

use player::Player;

pub struct LudoGame {
    engine: LudoEngine,  // actual logic
    active_players: Vec<Player>, // order matters !
}

impl LudoGame {
    pub fn take_intro(display: &mut Display) -> LudoGame {
        let msg = "Enter names of the Players (Leave empty if not playing, or type \"ROBOT\") :";

        "Player %d";

        todo!()
    }

    pub fn play(&mut self, display: &mut Display) {

        todo!()
    }
}
