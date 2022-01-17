use std::cell::RefCell;

use crate::engine::{Rang, Goti};

pub struct Player {
    pub name: String,
    pub colour: Rang,
    pub moving_gotis: Vec<RefCell<Goti>>,
    pub locked_gotis: Vec<RefCell<Goti>>,
    pub num_finished: u8
}
