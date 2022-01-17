use std::cell::RefCell;
use super::goti::LudoGoti;
use super::rang::Rang;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LudoCellType {
    Default,
    SafeSpot,
    LockedPosition(Rang),
    HomeLane(Rang),

    NoUse   // MUST not be mutated, such a cell will panic on invalid (eg. movedHere etc.)
}

#[derive(Clone)]
pub struct LudoCell {
    pub cell_type: LudoCellType,
    pub gotis: Vec<RefCell<LudoGoti>>
}

impl LudoCell {
    pub fn is_safe_spot(&self) -> bool {
        self.cell_type == LudoCellType::SafeSpot
    }
}
