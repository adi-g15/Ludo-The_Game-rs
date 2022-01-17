use std::{collections::BTreeMap as Map, cell::Cell};

mod cell;
mod rang;
mod goti;

pub use rang::Rang as Colour;
use cell::LudoCell as Box;
use goti::LudoGoti as Goti;

pub struct LudoEngine {
    board: [[Box; 15]; 15],
    locked_positions: Map<Colour,[(u16,u16); 4]>,
    moving_gotis: Map<Colour, Vec<Cell<Goti>>>,
    locked_gotis: Map<Colour, Vec<Cell<Goti>>>,
    active_colours: Vec<Colour>
}

impl LudoEngine {

}
