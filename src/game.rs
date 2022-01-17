mod player;

use std::io::{stdin, Read, stdout, Write};

use crate::engine::{LudoEngine, Rang, dice::roll};
use crate::display::Display;

use crossterm::style::Color;
use player::Player;

pub struct LudoGame {
    engine: LudoEngine,  // actual logic
    display: Display,
    active_players: Vec<Player>, // order matters !
}

impl LudoGame {
    pub fn new() -> Self {
        let mut display = Display::new();
        let mut active_players = Vec::new();
        let mut active_colours = Vec::new();

        let player_names = display.get_player_names();
        let colors = [Rang::Red, Rang::Green, Rang::Yellow, Rang::Blue];

        for (i,name) in player_names.iter().enumerate() {
            if ! name.is_empty() {
                active_colours.push(colors[i]);
            }
        }

        let engine = LudoEngine::new(active_colours);

        for (i,name) in player_names.iter().enumerate() {
            if ! name.is_empty() {
                active_players.push(
                    Player {
                        name: name.clone(),
                        colour: colors[i],
                        moving_gotis: Vec::new(),
                        locked_gotis: engine.get_locked_gotis(colors[i]),
                        num_finished: 0,
                    }
                )
            }
        }

        if active_players.is_empty() {
            Display::splash_screen("No players entered", Some(Color::Red));
            std::thread::sleep(std::time::Duration::from_secs(10));
            panic!("No players entered");
        }

        LudoGame {
            active_players,
            display,
            engine
        }
    }

    fn update_display(&self) {
        // `display` component requires this
        let mut display_content = Vec::new();
        let board = &self.engine.get_board();

        for (i, row) in board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if ! cell.gotis.is_empty() {
                    // Invariant: Assuming all gotis in one cell, even if multiple, are of same color
                    let mut content = match cell.gotis[0].borrow().colour {
                        Rang::Red => '🔴',
                        Rang::Green => '🟢',
                        Rang::Yellow => '🟡',
                        Rang::Blue => '🔵'
                    }.to_string();

                    if cell.gotis.len() > 1 { 
                        content.push_str(&cell.gotis.len().to_string())
                    }

                    display_content.push(((i as u8,j as u8), content));

                    // Note: Not handling case of multiple gotis of different colors, in same cell, eg. "RG", "RGRB" which should be shown as "R2GB"
                }
            }
        }

        self.display.update_display(display_content);
    }

    pub fn play(&mut self) {
        self.update_display();

        loop {
            for player in self.active_players.iter() {
                if self.engine.is_finished(player.colour) {
                    continue;
                }
                self.engine.set_current_colour(player.colour);
                self.display.set_player(&player.name);
                self.update_display();

                print!("Press Enter to Roll: ");
                stdout().flush();
                // ignore input till Enter
                let mut ignore_buf = String::new();
                stdin().read_line(&mut ignore_buf);
                ignore_buf.clear();

                let mut dice_numbers = Vec::new();

                while *dice_numbers.last().unwrap_or(&6) == 6 {
                    dice_numbers.push(roll())
                }

                println!("Roll Outputs - {:?}", dice_numbers);

                /*
                Chose from these gotis : 
0. Unlock New Goti (just type 0)

1. [3][8]

Roll Output - 6 4 
Enter Goti and dieNumber : 

                */

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        // !TODO
    }
}

impl Drop for LudoGame {
    fn drop(&mut self) {
        self.display.end_display();
    }
}
