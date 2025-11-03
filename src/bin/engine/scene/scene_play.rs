use super::{SceneContinuation};

use std::collections::HashMap;

use crate::engine::map::Map;
use crate::engine::obj::ObjProperties;
use crate::engine::input::Input;
use crate::engine::for_scripting::Cmd;

/// Interactive map, the actual gameplay part of the game.
#[derive(Clone, Debug)]
pub struct Play {
    // Layout of current map.
    pub field: Map,
}

impl Play
{
    // TODO: Do we need a function or would having levset_biobots use Play {...} be better?
    // TODO: Use lifetime or Rc on map_key instead of clone()?
    pub fn from_ascii<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<ObjProperties>>,
    ) -> Play {
        Play {
            field: Map::from_map_and_key(ascii_map, map_key),
        }
    }

    pub fn advance<Scripts: super::super::for_scripting::BaseScripts>(&mut self, input : &mut Input) -> SceneContinuation  {
        let cmd = input.consume_cmd().unwrap_or(Cmd::default_cmd());
        self.field.advance::<Scripts>(cmd)
    }
}
