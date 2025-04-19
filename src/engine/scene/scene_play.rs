use super::{SceneEnding};

use std::collections::HashMap;

use crate::engine::field::Field;
use crate::engine::obj::Obj;
use crate::engine::input::Input;
use crate::engine::scripting::Cmd;

/// Interactive map, the actual gameplay part of the game.
#[derive(Clone, Debug)]
pub struct Play {
    // Layout of current map.
    pub field: Field,
}

impl Play
{
    // TODO: Do we need a function or would having levset_biobots use Play {...} be better?
    // TODO: Use lifetime or Rc on map_key instead of clone()?
    pub fn from_ascii<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<Obj>>,
    ) -> Play {
        Play {
            field: Field::from_map_and_key(ascii_map, map_key),
        }
    }

    pub fn advance(&mut self, input : &mut Input) -> SceneEnding  {
        let cmd = input.consume_cmd().unwrap_or(Cmd::default_cmd());
        self.field.advance(cmd)
    }
}
