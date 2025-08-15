use serde_repr::{Serialize_repr, Deserialize_repr};
use num_enum::TryFromPrimitive;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Tile {
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



