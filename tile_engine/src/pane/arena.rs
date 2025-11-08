use super::{PaneContinuation};
use super::BasePane;

use std::collections::HashMap;

use crate::map::Map;
use crate::obj::FreeObj;
use crate::input::Input;
use crate::for_gamedata::Cmd;

/// Interactive map, the actual gameplay part of the game.
/// Breadcrumb: Could merge Map into here.
#[derive(Clone, Debug)]
pub struct Arena<MovementLogic: super::super::for_gamedata::BaseMovementLogic> {
    // Layout of current map.
    pub map: Map<MovementLogic>,
}

impl<MovementLogic : super::super::for_gamedata::BaseMovementLogic> BasePane for Arena<MovementLogic>
{
    fn advance(&mut self, input : &mut Input) -> PaneContinuation  {
        let cmd = input.consume_cmd().unwrap_or(Cmd::default());
        self.map.advance(cmd)
    }

    fn need_sync_to_ticks(&self) -> bool {
        true
    }
}

impl<MovementLogic: super::super::for_gamedata::BaseMovementLogic> Arena<MovementLogic>
{
    pub fn from_ascii<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
    ) -> Self {
        Self {
            map: Map::from_map_and_key(ascii_map, map_key),
        }
    }
}
