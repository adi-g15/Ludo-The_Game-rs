use array_init::array_init;
use std::{cell::RefCell, collections::BTreeMap as Map};

pub mod dice;
mod cell;
mod goti;
mod rang;

use self::{
    cell::{LudoCell as Box, LudoCellType},
    goti::LudoGoti,
};
pub use rang::Rang; // 'Colour' use krne se confusion hota h kyunki crossterm::Color me bhi h

pub struct LudoEngine {
    board: [[Box; 15]; 15],
    locked_positions: Map<Rang, [(u8, u8); 4]>,
    moving_gotis: Map<Rang, Vec<RefCell<LudoGoti>>>,
    locked_gotis: Map<Rang, Vec<RefCell<LudoGoti>>>,
    active_colours: Vec<Rang>,
    curr_colour: Rang
}

impl LudoEngine {
    pub fn new(active_colours: Vec<Rang>) -> Self {
        if active_colours.is_empty() {
            panic!("No active colours to play 🥲");
        }

        // Bas is array ko initialise krne ke liye itna krna pda :')
        let mut board: [[Box; 15]; 15] = array_init(|i| {
            array_init(|j| Box {
                cell_type: LudoCellType::NoUse,
                gotis: Vec::new(),
            })
        });

        // Order Matters: For eg. Default also marks some as SafeSpots
        // Defaults (ie. usual Path box)
        {
            for r in 6..9 {
                for c in 0..15 {
                    board[r][c].cell_type = LudoCellType::Default;
                    board[c][r].cell_type = LudoCellType::Default;
                }
            }

            // Mark middle square as NoUse again
            for r in 6..9 {
                for c in 6..9 {
                    board[r][c].cell_type = LudoCellType::NoUse;
                }
            }
        }

        // Safe Spots
        for (r, c) in [
            (1, 8),
            (2, 6),
            (6, 1),
            (6, 12),
            (8, 2),
            (8, 13),
            (12, 8),
            (13, 6),
        ] {
            board[r][c].cell_type = LudoCellType::SafeSpot;
        }

        let mut locked_positions = Map::new();
        for colour in active_colours.iter() {
            locked_positions.insert(
                *colour,
                match *colour {
                    Rang::Red => [(10, 1), (10, 4), (13, 1), (13, 4)],
                    Rang::Green => [(1, 1), (1, 4), (4, 1), (4, 4)],
                    Rang::Yellow => [(1, 10), (1, 13), (4, 10), (4, 13)],
                    Rang::Blue => [(10, 10), (10, 13), (13, 10), (13u8, 13u8)]
                }
            );
        }
        //: fun note: the u16 at end is just to fool/tell the rust compiler that the value is (u16,u16),
        //  not (usize,usize) which it earlier detected (it waws doing so, because it was used to index an array, for which u16 or u8 can NOT be used)

        let mut locked_gotis = Map::new();
        // Locked Locations:
        for (colour, coords) in locked_positions.iter() {
            locked_gotis.insert(*colour, Vec::new());

            for (r, c) in coords {
                board[*r as usize][*c as usize].cell_type = LudoCellType::LockedPosition(*colour);

                let goti_ref = RefCell::new(LudoGoti { colour: *colour });

                locked_gotis
                    .get_mut(colour).unwrap()
                    .push(goti_ref.clone());

                // board should know also
                board[*r as usize][*c as usize].gotis.push(goti_ref);
            }
        }

        // Home Lane (the ending path to finish)
        {
            // Red
            for r in 9..14 {
                board[r][7].cell_type = LudoCellType::HomeLane(Rang::Red);
            }

            // Green
            for c in 1..6 {
                board[7][c].cell_type = LudoCellType::HomeLane(Rang::Green);
            }

            // Yellow
            for r in 1..6 {
                board[r][7].cell_type = LudoCellType::HomeLane(Rang::Yellow);
            }

            // Blue
            for c in 9..14 {
                board[7][c].cell_type = LudoCellType::HomeLane(Rang::Blue);
            }
        }

        let mut moving_gotis = Map::new();
        moving_gotis.insert(Rang::Red, Vec::new());
        moving_gotis.insert(Rang::Green, Vec::new());
        moving_gotis.insert(Rang::Yellow, Vec::new());
        moving_gotis.insert(Rang::Blue, Vec::new());

        LudoEngine {
            curr_colour: active_colours[0],
            active_colours,
            board,
            locked_gotis,
            locked_positions,
            moving_gotis,
        }
    }

    pub fn get_board(&self) -> &[[Box;15];15] {
        &self.board
    }

    pub fn get_locked_gotis(&self, rang: Rang) -> Vec<RefCell<LudoGoti>> {
        match self.locked_gotis.get(&rang) {
            Some(v) => v.clone(),
            None => panic!("Tried to get_locked_gotis for non-playing colour: {:?}", rang)
        }
    }

    /** @note Will always return `true` for a colour that is not playing */
    pub(crate) fn is_finished(&self, colour: Rang) -> bool {
        if self.active_colours.contains(&colour) == false {
            true
        } else {
            self.moving_gotis.get(&colour).unwrap().is_empty() &&
            self.locked_gotis.get(&colour).unwrap().is_empty()
        }
    }

    pub(crate) fn set_current_colour(&mut self, colour: Rang) {
        self.curr_colour = colour;
    }
}

// Exports
pub use goti::LudoGoti as Goti;
