use super::{SceneEnding};

// TODO: Better to have as template or function parameter than as import
use crate::game_helpers::*;

// Would be nice to remove if easy
use macroquad::prelude::*;

use std::collections::HashMap;

use crate::engine::field::Field;
use crate::engine::obj::Obj;

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
        // TODO: Get size from strings. Assert equal to default 16 in meantime.
        let mut play = Play {
            field: Field {
                map_key: map_key.clone(),
                ..Field::empty(ascii_map[0].len() as u16, HEIGHT as u16)
            },
        };

        for (y, line) in ascii_map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                for ent in map_key.get(&ch).unwrap() {
                    play.spawn_at(x as i16, y as i16, ent.clone());
                }
            }
        }

        play
    }

    /// Add ent to map.
    ///
    /// Might not be necessary as a separate fn now roster logic in field.
    pub fn spawn_at(&mut self, x: i16, y: i16, orig_obj: Obj) {
        self.field.place_obj_at(x, y, orig_obj);
    }

    /// TODO: Try to extract to object-type specific parts to function in game-helper directory
    pub fn advance(&mut self, last_key_pressed: Option<KeyCode>) -> SceneEnding  {
        // FIXME: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // STUB: Maybe display char moving out of sync with enemy.

        // Before movement, reset "prev". Will be overwritten if movement happens.
        self.field.map[self.field.roster.hero].prev_pos = self.field.map[self.field.roster.hero].cached_pos;

        move_character(&mut self.field, last_key_pressed)?;

        // Move all movs
        for mov in &mut self.field.roster.movs {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            self.field.map[*mov].prev_pos = self.field.map[*mov].cached_pos;

            move_mov(&mut self.field.map, &self.field.roster.hero, mov)?;
        }
        SceneEnding::Continue(())
    }

}
