use crate::actions::{Action, requires_tile};
use crate::tiles::Tile;

#[derive(Clone, Copy)]
pub struct TileOrAction {
    inner: u8
}

const ACTION_DEF: u8 = 0b1110_0000;
impl TileOrAction {
    #[inline(always)]
    pub const fn is_tile(&self) -> bool {
        // the high three bits of every tile are 000, and all actions contain a nonzero bit among them
        !self.inner & ACTION_DEF == ACTION_DEF
    }

    #[inline(always)]
    pub const fn is_action(&self) -> bool {
        !self.is_tile()
    }

    pub(crate) fn new_unchecked(value: u8) -> Self {
        Self {
            inner: value
        }
    }

    pub fn value_unchecked(&self) -> (Action, Tile) {
        if self.is_tile() { 
            return (Action::None, Tile::try_from(self.inner).unwrap()) 
        }
        
        if requires_tile(self.inner) {
            return (Action::try_from(self.inner & 0b1110_0000).unwrap(),
             Tile::try_from(self.inner & 0b0001_1111).unwrap())
        }

        (Action::try_from(self.inner).unwrap(), Tile::None)
    }
}
