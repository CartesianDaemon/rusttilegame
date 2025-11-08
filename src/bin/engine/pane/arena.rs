use super::{PaneContinuation};
use super::BasePane;

use std::collections::HashMap;

use crate::engine::map::Map;
use crate::engine::obj::FreeObj;
use crate::engine::input::Input;
use crate::engine::for_scripting::Cmd;

/// Interactive map, the actual gameplay part of the game.
#[derive(Clone, Debug)]
pub struct Arena<MovementLogic: super::super::for_scripting::BaseMovementLogic> {
    // Layout of current map.
    // TODO: Rename map to map
    pub map: Map<MovementLogic>,
}

impl<MovementLogic : super::super::for_gamedata::BaseMovementLogic> BasePane for Arena<MovementLogic>
{
    fn advance(&mut self, input : &mut Input) -> PaneContinuation  {
        let cmd = input.consume_cmd().unwrap_or(Cmd::default_cmd());
        self.map.advance(cmd)
    }

    fn is_continuous(&self) -> bool {
        false
    }
}

impl<MovementLogic: super::super::for_scripting::BaseMovementLogic> Arena<MovementLogic>
{
    // TODO: Do we need a function or would having levset_biobots use Arena {...} be better?
    // TODO: Use lifetime or Rc on map_key instead of clone()?
    // TODO: Could Map be merged into this class?
    pub fn from_ascii<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
    ) -> Self {
        Self {
            map: Map::from_map_and_key(ascii_map, map_key),
        }
    }
}
