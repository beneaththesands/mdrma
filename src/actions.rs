use serde_repr::{Serialize_repr, Deserialize_repr};
use num_enum::TryFromPrimitive;

const NEEDS_TILE_DEF: u8 = 0b1110_0000;
#[inline(always)]
pub fn requires_tile(action: u8) -> bool {
    action & NEEDS_TILE_DEF == NEEDS_TILE_DEF
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Action {
    // Not a valid action, and different than Tile::None in case we accidentally store it
    None = 0b1111_1111,

    // Leading 10 is Chii
    // A complete chii action will specify the lowest tile in the seqence called in the lowest 5 bits.
    CallChii = 0b1000_0000,
    CallChiiWithRedFive = 0b1010_0000,

    // Then the other commands that need to specify full tiles
    // May have multiple pons to convert, need to specify tile
    DeclareConvertedKan = 0b0010_0000,
    // Specify arbitrary tile for discard
    DeclareRiichi = 0b0100_0000,
    // May have multiple sets eligible
    DeclareClosedKan = 0b0110_0000,

    // The rest of these will be followed by a discard, but need to specify the player making the call.
    // All remaining commands have 111 high bits for clarity. The final 2 bits of these command specify a player.
    // 111 000 is Kan
    CallKanByEast = 0b1110_0000,
    CallKanBySouth = 0b1110_0001,
    CallKanByWest = 0b1110_0010,
    CallKanByNorth = 0b1110_0011,

    // 111 001 is Pon
    // 111 010 is Pon that contains a red five from the hand
    CallPonByEast = 0b1110_0100,
    CallPonBySouth = 0b1110_0101,
    CallPonByWest = 0b1110_0110,
    CallPonByNorth = 0b1110_0111,
    CallPonByEastWithRedFive = 0b1110_1000,
    CallPonBySouthWithRedFive = 0b1110_1001,
    CallPonByWestWithRedFive = 0b1110_1010,
    CallPonByNorthWithRedFive = 0b1110_1011,
    // 111 011 is Ron
    CallRonByEast = 0b1110_1100,
    CallRonBySouth = 0b1110_1101,
    CallRonByWest = 0b1110_1110,
    CallRonByNorth = 0b1110_1111,

    // All remaining commands have leading 1111. 
    DeclareKita = 0b1111_0000,
    DeclareTsumo = 0b1111_0001,
    DeclareDraw = 0b1111_0010,
}