use serde_repr::{Serialize_repr, Deserialize_repr};
use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, Hash)]
#[repr(u8)]
pub enum Player {
    Right = 0b00,
    Opposite = 0b01,
    Left = 0b10,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive, Default, Hash)]
#[repr(u8)]
pub enum Action {
    #[default]
    // Not a valid action, and different than Tile::None in case we accidentally store it
    None = 0b1111_1111,

    // Leading 01 is Chii or Kan
    // Called Chi and Declared Kans cannot occur at the same point in the game, and so can be safely parsed.
    // Converted Kan and Closed Kan are represented together and need to be parsed by the client
    // In the cases where a meld can optionally contain a hidden red five, we need to indicate if that occured.
    // Because one cannot chii on a honor tile, it is safe to adopt the third highest bit as a flag for this case
    // A complete chii action will specify the lowest tile in the sequence called in the lowest 5 bits.
    CallChiiOrDeclareKan = 0b0100_0000,
    CallChiiWithRedFive = 0b0110_0000,

    // Leading 10 is Riichi, specifying an arbitrary tile for discard
    DeclareRiichi = 0b1000_0000,

    // All remaining commands have 11 high bits for clarity.
    // 1110 10 is Kan
    CallKanByRight = 0b1110_1000,
    CallKanByOpposite = 0b1110_1001,
    CallKanByLeft = 0b1110_1010,

    // 1110 11 is Pon
    CallPonByRight = 0b1110_1100,
    CallPonByOpposite = 0b1110_1101,
    CallPonByLeft = 0b1110_1110,
    
    // 1111 01 is Pon with an optional red five
    CallPonByRightWithRedFive = 0b1111_0100,
    CallPonByOppositeWithRedFive = 0b1111_0101,
    CallPonByLeftWithRedFive = 0b1111_0110,

    // 1111 10 is Ron
    CallRonByRight = 0b1111_1000,
    CallRonByOpposite = 0b1111_1001,
    CallRonByLeft = 0b1111_1010,

    // 1111 11 are the remaining declarations made from hand
    DeclareKita = 0b1111_1100,
    DeclareTsumo = 0b1111_1101,
    DeclareMulligan = 0b1111_1110,
}

impl Action {
    // Chi, Closed/Converted Kan, and Riichi have high bits that are less than 3.
    #[inline(always)]
    pub fn has_tile(action: u8) -> bool {
        action >> 6 < 3
    }

    #[inline(always)]
    pub fn has_player(&self) -> bool {
        let raw = *self as u8;
        !Action::has_tile(raw) & (raw >> 2 < 0b11_1111)
    }

    pub fn get_player_unchecked(&self) -> Player {
        Player::try_from(*self as u8 & 0b0000_0011).unwrap()
    }

    pub fn get_player(&self) -> Option<Player> {
        if self.has_player() {
            return Some(self.get_player_unchecked())
        }

        None
    }
}

impl From<Action> for u8 {
    fn from(value: Action) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod test {
    use crate::actions::*;
    
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Action>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Action>();
    }

    #[test]
    fn validate() {
        check_expect_action(Action::None, false, None);
        check_expect_action(Action::CallChiiOrDeclareKan, false, None);
        check_expect_action(Action::CallChiiWithRedFive, false, None);
        check_expect_action(Action::DeclareRiichi, false, None);
        check_expect_action(Action::CallKanByRight, true, Some(Player::Right));
        check_expect_action(Action::CallKanByOpposite, true, Some(Player::Opposite));
        check_expect_action(Action::CallKanByLeft, true, Some(Player::Left));
        check_expect_action(Action::CallPonByRight, true, Some(Player::Right));
        check_expect_action(Action::CallPonByOpposite, true, Some(Player::Opposite));
        check_expect_action(Action::CallPonByLeft, true, Some(Player::Left));
        check_expect_action(Action::CallPonByRightWithRedFive, true, Some(Player::Right));
        check_expect_action(Action::CallPonByOppositeWithRedFive, true, Some(Player::Opposite));
        check_expect_action(Action::CallPonByLeftWithRedFive, true, Some(Player::Left));
        check_expect_action(Action::CallRonByRight, true, Some(Player::Right));
        check_expect_action(Action::CallRonByOpposite, true, Some(Player::Opposite));
        check_expect_action(Action::CallRonByLeft, true, Some(Player::Left));
        check_expect_action(Action::DeclareKita, false, None);
        check_expect_action(Action::DeclareTsumo, false, None);
        check_expect_action(Action::DeclareMulligan, false, None);
    }

    fn check_expect_action(action: Action, should_have_player: bool, expected_player:Option<Player>) {
        let has_player = action.has_player();
        assert_eq!(has_player, should_have_player);
        if has_player {
            assert_eq!(action.get_player().unwrap(), expected_player.unwrap());
            assert_eq!(action.get_player_unchecked(), expected_player.unwrap());
        }
        else {
            assert!(action.get_player().is_none());
            assert!(expected_player.is_none());
        }
    }

    #[test]
    fn validate_tiles() {
        check_expect_tile(Action::None, false);
        check_expect_tile(Action::CallChiiOrDeclareKan, true);
        check_expect_tile(Action::CallChiiWithRedFive, true);
        check_expect_tile(Action::DeclareRiichi, true);
        check_expect_tile(Action::CallKanByRight, false);
        check_expect_tile(Action::CallKanByOpposite, false);
        check_expect_tile(Action::CallKanByLeft, false);
        check_expect_tile(Action::CallPonByRight, false);
        check_expect_tile(Action::CallPonByOpposite, false);
        check_expect_tile(Action::CallPonByLeft, false);
        check_expect_tile(Action::CallPonByRightWithRedFive, false);
        check_expect_tile(Action::CallPonByOppositeWithRedFive, false);
        check_expect_tile(Action::CallPonByLeftWithRedFive, false);
        check_expect_tile(Action::CallRonByRight, false);
        check_expect_tile(Action::CallRonByOpposite, false);
        check_expect_tile(Action::CallRonByLeft, false);
        check_expect_tile(Action::DeclareKita, false);
        check_expect_tile(Action::DeclareTsumo, false);
        check_expect_tile(Action::DeclareMulligan, false);
    }

    fn check_expect_tile(action: Action, should_have_tile: bool) {
        assert_eq!(Action::has_tile(action as u8), should_have_tile);
    }

}