use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize};

mod tiles;
mod actions;
mod tile_or_action;

use crate::tile_or_action::TileOrAction;
pub use crate::tiles::Tile;
pub use crate::actions::Action;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Default, Debug, Hash)]
pub struct Hand {
    #[serde(rename="i")] 
    initial_state: InitialState,
    #[serde(default, rename="a", skip_serializing_if = "crate::is_default")]
    actions: Vec<u8>
}

impl Hand {
    pub fn new() -> Self {
        Hand::default()
    }
    pub fn new_from_unchecked(init: InitialState) -> Self {
        Self {
            initial_state: init,
            actions: Vec::default()
        }
    }

    pub fn draw_unchecked(&mut self, tile: Tile) -> &mut Self {
        self.actions.push(tile as u8);
        self
    }

    pub fn discard_unchecked(&mut self, tile: Tile) -> &mut Self {
        self.actions.push(tile as u8);
        self
    }

    pub fn act_unchecked(&mut self, action: Action, tile: Option<Tile>) -> &mut Self {
        let mut stored = action as u8;
        if tile.is_some() {
            stored |= tile.unwrap() as u8;
        }
        self.actions.push(stored as u8);
        self
    }

    pub fn initial_state(&self) -> &InitialState {
        &self.initial_state
    }

    pub fn to_parts(self) -> (InitialState, impl Iterator<Item = TileOrAction>) {
        (self.initial_state, self.actions.into_iter().map(|item| TileOrAction::new_unchecked(item)))
    }
}

#[inline]
fn is_default<T>(t: &T) -> bool where T : Default+PartialEq {
    *t == T::default() 
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
#[repr(u8)]
pub enum Wind {
    #[default]
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default, Hash)]
pub struct InitialState {
    #[serde(rename="e")] 
    pub east_hand: Vec<Tile>,
    #[serde(rename="s")] 
    pub south_hand: Vec<Tile>,
    #[serde(rename="w")] 
    pub west_hand: Vec<Tile>,
    #[serde(rename="n")] 
    pub north_hand: Vec<Tile>,
    #[serde(rename="d")] 
    pub dead_wall: Vec<Tile>,
    #[serde(rename="t")] 
    pub living_wall: Vec<Tile>,
    #[serde(default, rename="x", skip_serializing_if = "crate::is_default")] 
    pub repeat_count: u8,
    #[serde(default, rename="h", skip_serializing_if = "crate::is_default")] 
    pub hanba_count:u8,
    #[serde(default, rename="r", skip_serializing_if = "crate::is_default")] 
    pub unclaimed_riichi_count:u8,
    #[serde(rename="p")] 
    pub prevailing_wind: Wind,
}

impl InitialState {
    pub fn new() -> Self {
        InitialState::default()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use rand::prelude::SliceRandom;
    use rand_xoshiro::Xoshiro256StarStar;
    use rand::SeedableRng;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Hand>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Hand>();
    }

    #[test]
    fn basics() {
        do_serialize(&Hand::default());
    }

    #[test]
    fn valid_random_setup() {
        let mut init = empty_init();
        let mut tiles = get_tiles();
        let mut rng =  Xoshiro256StarStar::from_seed([
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        tiles.shuffle(&mut rng);
        init_tiles(tiles, &mut init);
        do_serialize(&Hand::new_from_unchecked(init));
    }

    #[test]
    fn valid_stable_setup() {
        let mut init = empty_init();
        init.prevailing_wind = Wind::South;
        init.hanba_count = 3;
        init.repeat_count = 2;
        init.unclaimed_riichi_count = 1;
        init_tiles(get_tiles(), &mut init);

        do_serialize(&Hand::new_from_unchecked(init));
    }

    #[test]
    fn unchecked_invalid_stable_game() {
        let mut init = empty_init();
        let mut tiles = get_tiles();
        let mut rng =  Xoshiro256StarStar::from_seed([
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        tiles.shuffle(&mut rng);
        init_tiles(tiles, &mut init);

        let mut hand = Hand::new_from_unchecked(init);
        let living_wall = (*(&hand.initial_state.living_wall)).clone();

        for tile in living_wall {
            hand.draw_unchecked(tile)
                .discard_unchecked(tile)
                .act_unchecked(Action::CallChiiOrDeclareKan, Some(tile));
        }

        do_serialize(&hand);
    }

    fn do_serialize(hand: &Hand) {
        let mut serial = vec![];
        ciborium::into_writer(&hand, &mut serial).ok();

        let output: Hand = ciborium::from_reader(&serial[..]).unwrap();
        let for_display = hex::encode(&serial);
        let len = serial.len();
        println!("{:?}", hand);
        println!("{:?}", for_display);
        println!("Byte count: {len}");
        println!("{:?}", output);
        assert_eq!(*hand, output);
    }

    fn init_tiles(mut tiles: Vec<Tile>, state: &mut InitialState) {
        state.east_hand = tiles.split_off(tiles.len() - 13);
        state.south_hand = tiles.split_off(tiles.len() - 13);
        state.west_hand = tiles.split_off(tiles.len() - 13);
        state.north_hand = tiles.split_off(tiles.len() - 13);
        state.dead_wall = tiles.split_off(tiles.len() - 14);
        state.living_wall = tiles;
    }

    fn empty_init() -> InitialState {
        InitialState {
            repeat_count: 0,
            unclaimed_riichi_count: 0,
            hanba_count: 0,
            prevailing_wind: Wind::East,
            east_hand: vec![],
            south_hand: vec![],
            west_hand: vec![],
            north_hand: vec![],
            dead_wall: vec![],
            living_wall: vec![],
        }
    }

    fn get_tiles() -> Vec<Tile> {
        use tiles::Tile::*;
        vec![
            PinOne,
            PinTwo,
            PinThree,
            PinFour,
            PinFive,
            PinSix,
            PinSeven, 
            PinEight, 
            PinNine,
            PinOne,
            PinTwo,
            PinThree,
            PinFour,
            PinFive,
            PinSix,
            PinSeven, 
            PinEight, 
            PinNine,
            PinOne,
            PinTwo,
            PinThree,
            PinFour,
            PinFive,
            PinSix,
            PinSeven, 
            PinEight, 
            PinNine,
            PinOne,
            PinTwo,
            PinThree,
            PinFour,
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
            SouSix,
            SouSeven,
            SouEight,
            SouNine,
            SouOne,
            SouTwo,
            SouThree,
            SouFour,
            SouFive,
            SouSix,
            SouSeven,
            SouEight,
            SouNine,
            SouOne,
            SouTwo,
            SouThree,
            SouFour,
            SouFive,
            SouSix,
            SouSeven,
            SouEight,
            SouNine,
            SouOne,
            SouTwo,
            SouThree,
            SouFour,
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
            ManSix,
            ManSeven,
            ManEight,
            ManNine,
            ManOne,
            ManTwo,
            ManThree,
            ManFour,
            ManFive,
            ManSix,
            ManSeven,
            ManEight,
            ManNine,
            ManOne,
            ManTwo,
            ManThree,
            ManFour,
            ManFive,
            ManSix,
            ManSeven,
            ManEight,
            ManNine,
            ManOne,
            ManTwo,
            ManThree,
            ManFour,
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
            HonorEast,
            HonorSouth,
            HonorWest,
            HonorNorth,
            HonorRedDragon,
            HonorWhiteDragon,
            HonorGreenDragon,
            HonorEast,
            HonorSouth,
            HonorWest,
            HonorNorth,
            HonorRedDragon,
            HonorWhiteDragon,
            HonorGreenDragon,
            HonorEast,
            HonorSouth,
            HonorWest,
            HonorNorth,
            HonorRedDragon,
            HonorWhiteDragon,
            HonorGreenDragon
        ]
    }
}
