use crate::actions::*;
use crate::tiles::Tile;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TileOrAction {
    inner: u8
}

impl TileOrAction {
    #[inline(always)]
    pub const fn is_tile(&self) -> bool {
        // the high two bits of every tile are 00, and all actions contain a nonzero bit among them
        self.inner >> 6 == 0
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

    pub fn to_value_unchecked(&self, is_call: bool) -> (Action, Tile) {
        // There are a large number of commands that start with 11
        // Chii starts with 01, but can start with 011 for red fives
        if self.is_tile() { 
            return (Action::None, Tile::try_from(self.inner).unwrap()) 
        }

        let with_tile = Action::has_tile(self.inner);
        match (with_tile, is_call) {
            // Call with tile
            (true, true) => {
                return (
                    Action::try_from(self.inner & 0b0110_0000).unwrap(), 
                    Tile::try_from(self.inner & 0b1_1111).unwrap()
                )
            },
            // Declare with tile
            (true, false) => {
                return (
                    Action::try_from(self.inner & 0b1100_0000).unwrap(),
                    Tile::try_from(self.inner & 0b0011_1111).unwrap()
                )
            },
            // Tileless call
            (false, _) => {
                return (
                    Action::try_from(self.inner).unwrap(),
                    Tile::None
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tile_or_action::{TileOrAction};
    use crate::actions::*;
    use crate::tiles::*;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<TileOrAction>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<TileOrAction>();
    }

    #[test]
    fn validate() {
        check_value(Action::CallChiiOrDeclareKan, Tile::HonorGreenDragon, false);
        check_value(Action::CallChiiOrDeclareKan, Tile::PinOne, false);
        check_value(Action::CallChiiOrDeclareKan, Tile::PinOne, true);
        check_value(Action::CallChiiWithRedFive, Tile::ManFour, true);

        check_value(Action::DeclareRiichi, Tile::HonorSouth, false);
        check_value(Action::DeclareRiichi, Tile::SouNine, false);

        check_value(Action::CallPonByLeft, Tile::None, true);
        check_value(Action::CallPonByOppositeWithRedFive, Tile::None, true);
        check_value(Action::CallKanByRight, Tile::None, true);
        check_value(Action::CallRonByOpposite, Tile::None, true);

        check_value(Action::DeclareKita, Tile::None, false);
        check_value(Action::DeclareTsumo, Tile::None, false);
        check_value(Action::DeclareMulligan, Tile::None, false);

        check_value(Action::None, Tile::ManRedFive, false);
        check_value(Action::None, Tile::SouEight, false);
        check_value(Action::None, Tile::HonorGreenDragon, false);
        check_value(Action::None, Tile::HonorEast, false);
        check_value(Action::None, Tile::PinOne, false);
        check_value(Action::None, Tile::ManNine, false);
    }

    fn check_value(action: Action, tile: Tile, is_call: bool) {
        let action_value = if action == Action::None { !(action as u8) } else { action as u8 };
        let combo = TileOrAction::new_unchecked(action_value | tile as u8);
        let (out_action, out_tile) = combo.to_value_unchecked(is_call);
        assert_eq!(action, out_action);        
        assert_eq!(tile, out_tile);   
    }
}