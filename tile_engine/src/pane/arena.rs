use super::{PaneContinuation};
use super::BasePane;

use std::collections::HashMap;

use crate::map::Map;
use crate::obj::FreeObj;
use crate::input::Input;
use crate::for_scripting::Cmd;

/// Interactive map, the actual gameplay part of the game.
#[derive(Clone, Debug)]
pub struct Arena<MovementLogic: super::super::for_scripting::BaseMovementLogic> {
    // Layout of current map.
    pub map: Map<MovementLogic>,
}

impl<MovementLogic : super::super::for_gamedata::BaseMovementLogic> BasePane for Arena<MovementLogic>
{
    fn advance(&mut self, input : &mut Input) -> PaneContinuation  {
        let cmd = input.consume_cmd().unwrap_or(Cmd::default_cmd());
        self.map.advance(cmd)
    }

    fn need_sync_to_ticks(&self) -> bool {
        true
    }
}

impl<MovementLogic: super::super::for_scripting::BaseMovementLogic> Arena<MovementLogic>
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
