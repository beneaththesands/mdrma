use serde_repr::{Serialize_repr, Deserialize_repr};
use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Suit {
    Pin,
    Sou,
    Man,
    Wind,
    Dragon,
}

impl Suit {
    pub fn is_honor(&self) -> bool {
        self == &Suit::Wind || self == &Suit::Dragon
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default, TryFromPrimitive, Hash)]
#[repr(u8)]
pub enum Tile {
    #[default]
    None = 0,

    PinOne,
    PinTwo,
    PinThree,
    PinFour,
    PinFive,
    PinRedFive,
    PinSix,
    PinSeven, 
    PinEight, 
    PinNine,

    SouOne,
    SouTwo,
    SouThree,
    SouFour,
    SouFive,
    SouRedFive,
    SouSix,
    SouSeven,
    SouEight,
    SouNine,

    ManOne,
    ManTwo,
    ManThree,
    ManFour,
    ManFive,
    ManRedFive,
    ManSix,
    ManSeven,
    ManEight,
    ManNine,
  
    HonorEast,
    HonorSouth,
    HonorWest,
    HonorNorth,
    HonorRedDragon,
    HonorWhiteDragon,
    HonorGreenDragon,
}

impl Tile {
    pub fn suit(&self) -> Option<Suit> {
        if self == &Tile::None { return None }

        let value = (*self as u8 - 1) / 10;
        match value {
            0 => Some(Suit::Pin),
            1 => Some(Suit::Sou),
            2 => Some(Suit::Man),
            3 => {
                let num  = *self as u8;
                if num <= Tile::HonorNorth as u8 { 
                    return Some(Suit::Wind)
                }
                if num <= Tile::HonorGreenDragon as u8 {
                    return Some(Suit::Dragon)
                } 

                None 
            }
            _ => None,
        }
    }

    pub fn is_honor(&self) -> bool {
        self.suit().is_some_and(|suit| {
            suit.is_honor()
        })
    }

    pub(crate) fn is_five(&self) -> bool {
        match self {
            Tile::ManFive |
            Tile::ManRedFive |
            Tile::PinFive |
            Tile::PinRedFive |
            Tile::SouFive |
            Tile::SouRedFive => true,
            _ => false
        }
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.suit().and_then(|self_suit| other.suit().and_then(|other_suit| {
            if self_suit != other_suit { return None }
            if self == other { return Some(std::cmp::Ordering::Equal) }
            if self_suit.is_honor() || other_suit.is_honor() { return None }
            // All fives are created equal before scoring time
            if self.is_five() && other.is_five() { return Some(std::cmp::Ordering::Equal) }

            Some((*self as u8).cmp(&(*other as u8)))
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::tiles::*;
    use std::cmp::Ordering;
    use std::cmp::Ordering::*;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Tile>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Tile>();
    }

    #[test]
    fn test_send_suit() {
        fn assert_send<T: Send>() {}
        assert_send::<Suit>();
    }

    #[test]
    fn test_sync_suit() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Suit>();
    }

    #[test]
    fn validate_compare() {
        check_expect_compare(Tile::HonorEast, Tile::HonorNorth, None);
        check_expect_compare(Tile::HonorNorth, Tile::HonorNorth, Some(Equal));
        check_expect_compare(Tile::HonorGreenDragon, Tile::HonorNorth, None);
        check_expect_compare(Tile::HonorGreenDragon, Tile::HonorGreenDragon, Some(Equal));
        check_expect_compare(Tile::PinOne, Tile::PinTwo, Some(Less));
        check_expect_compare(Tile::PinFive, Tile::PinRedFive, Some(Equal));
        check_expect_compare(Tile::SouEight, Tile::SouNine, Some(Less));
        check_expect_compare(Tile::PinSeven, Tile::SouSeven, None);
        check_expect_compare(Tile::ManOne, Tile::SouSeven, None);
        check_expect_compare(Tile::PinOne, Tile::ManNine, None);
        check_expect_compare(Tile::SouSix, Tile::SouRedFive, Some(Greater));
        check_expect_compare(Tile::SouSix, Tile::SouFive, Some(Greater));
        check_expect_compare(Tile::ManRedFive, Tile::ManThree, Some(Greater));
        check_expect_compare(Tile::ManFive, Tile::ManTwo, Some(Greater));
    }

    fn check_expect_compare(left: Tile, right: Tile, expected_result: Option<Ordering>) {
        assert_eq!(left.partial_cmp(&right), expected_result);
        // We don't support total ordering, but our ordering is transitive and reflexive
        assert_eq!(right.partial_cmp(&left), expected_result.map(|result| result.reverse()));
    }

    #[test]
    fn validate_suit() {
        check_expect_suit(Tile::None, None, false);
        check_expect_suit(Tile::HonorEast, Some(Suit::Wind), true);
        check_expect_suit(Tile::HonorGreenDragon, Some(Suit::Dragon), true);
        check_expect_suit(Tile::ManFive, Some(Suit::Man), false);
        check_expect_suit(Tile::ManRedFive, Some(Suit::Man), false);
        check_expect_suit(Tile::PinOne, Some(Suit::Pin), false);
        check_expect_suit(Tile::SouNine, Some(Suit::Sou), false);
    }

    fn check_expect_suit(tile: Tile, expected_suit: Option<Suit>, expect_honor: bool) {
        assert_eq!(tile.is_honor(), expect_honor);
        assert_eq!(tile.suit(), expected_suit);
        if expect_honor { 
            assert!(tile.suit().is_some()) ;
            assert_eq!(tile.suit().unwrap().is_honor(), tile.is_honor());
        }
        else {
            assert!(tile.suit().is_none_or(|suit| !suit.is_honor()))
        }
    }
}



