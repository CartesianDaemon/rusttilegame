use super::Continuation;

// Would be nice to remove if easy
use macroquad::prelude::*;

use std::collections::HashMap;

use crate::engine::field::Field;
use crate::engine::obj::Obj;
use crate::engine::map_coords::*;

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
    pub fn advance(&mut self, last_key_pressed: Option<KeyCode>) -> Option<Continuation>  {
        // Need all the properties used in Ent.
        // May move "can move" like logic into load, along with the assorted properties.
        // While keeping movement code coordinating between ents here.
        use crate::engine::obj::*;

        // FIXME: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // STUB: Maybe display char moving out of sync with enemy.

        // Before movement, reset "prev". Will be overwritten if movement happens.
        self.field.map[self.field.ros.hero].prev_pos = self.field.map[self.field.ros.hero].cached_pos;

        // Move character
        if let Some(key) = last_key_pressed {
            let mut dir = CoordDelta::from_xy(0, 0);
            match key {
                KeyCode::Left  => dir = CoordDelta::from_xy(-1, 0),
                KeyCode::Right => dir = CoordDelta::from_xy(1, 0),
                KeyCode::Up    => dir = CoordDelta::from_xy(0, -1),
                KeyCode::Down  => dir = CoordDelta::from_xy(0, 1),
                _ => (),
            }
            if dir != CoordDelta::from_xy(0, 0) {
                if self.field.map.can_move(self.field.ros.hero, dir) {
                    self.field.map.move_delta(&mut self.field.ros.hero, dir);
                    // STUB: Check for win condition on ents other than the lowest one.
                    if self.field.map[MapHandle::from_xyh(self.field.ros.hero.x, self.field.ros.hero.y, 0)].effect == Effect::Win {
                        return Some(Continuation::PlayWin);
                    }
                }
            }
        }

        // Move all movs
        for bot in &mut self.field.ros.movs {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            self.field.map[*bot].prev_pos = self.field.map[*bot].cached_pos;

            match self.field.map[*bot].ai {
                AI::Stay => {
                    // Do nothing
                },
                AI::Hero => {
                    // Handled separately.
                },
                // STUB: When we see what mov movement logic are like, try to combine them into one fn.
                AI::Snake => {
                    // if mov on same row xor column as hero, change dir to face hero
                    if (bot.x == self.field.ros.hero.x) != (bot.y == self.field.ros.hero.y) {
                        let new_dir = CoordDelta::from_xy((self.field.ros.hero.x - bot.x).signum(),(self.field.ros.hero.y - bot.y).signum());
                        self.field.map[*bot].dir = new_dir;
                    }

                    // NOTE: When mov goes out of bounds is placeholder for real win condition.
                    if !(0..self.field.map.w() as i16).contains(&(bot.x + self.field.map[*bot].dir.dx)) ||
                        !(0..self.field.map.h() as i16).contains(&(bot.y + self.field.map[*bot].dir.dy))
                    {
                        return Some(Continuation::PlayWin);
                    }
                    else
                    {
                        // move mov to new location
                        // TODO: Have a "move_dir" fn.
                        let dir = self.field.map[*bot].dir;
                        self.field.map.move_delta(bot, dir);
                    }

                    // Die if mov moves onto hero
                    if bot.x == self.field.ros.hero.x && bot.y == self.field.ros.hero.y {
                        return Some(Continuation::PlayDie);
                    }
                },
                AI::Bounce => {
                    // TODO: Make a Map:: fn for "at pos + dir, or appropriate default if off map"

                    // If hitting wall, reverse direction.
                    if self.field.map.loc_at(*bot + self.field.map[*bot].dir).impassable() {
                        self.field.map[*bot].dir = CoordDelta::from_xy(-self.field.map[*bot].dir.dx, -self.field.map[*bot].dir.dy);
                    }

                    // Move. Provided next space is passable. If both sides are impassable, don't
                    // move.
                    if self.field.map.loc_at(*bot + self.field.map[*bot].dir).passable() {
                        self.field.map.move_delta(bot, self.field.map[*bot].dir);
                    }

                    // Hero dies if mov moves onto hero
                    if self.field.map[*bot].effect == Effect::Kill {
                        if bot.x == self.field.ros.hero.x && bot.y == self.field.ros.hero.y {
                            return Some(Continuation::PlayDie);
                        }
                    }
                },
                AI::Drift => {
                    // TODO: Deal with collisions between movs

                    let mut drift_dir = CoordDelta::from_xy(0, 0);
                    // If hitting wall, reverse direction.
                    if self.field.map.loc_at(*bot + self.field.map[*bot].dir).impassable() {
                        self.field.map[*bot].dir = CoordDelta::from_xy(-self.field.map[*bot].dir.dx, -self.field.map[*bot].dir.dy);
                        // If hero "visible" forward or sideways, move one sideways towards them, if passable.
                        // TODO: Check for obstacles to vision.
                        let hero_dir = CoordDelta::from_xy((self.field.ros.hero.x - bot.x).signum(),(self.field.ros.hero.y - bot.y).signum());
                        if self.field.map[*bot].dir.dx == 0 {
                            if hero_dir.dy != -self.field.map[*bot].dir.dy {
                                drift_dir = CoordDelta::from_xy(hero_dir.dx, 0);
                            }
                        } else if self.field.map[*bot].dir.dy == 0 {
                            if hero_dir.dx != -self.field.map[*bot].dir.dx {
                                drift_dir = CoordDelta::from_xy(0, hero_dir.dy);
                            }
                        } else {
                            panic!("AI::Drift only implemented for orthogal movement");
                        }
                    }

                    // Move. Provided next space is passable. If both sides are impassable, don't move.
                    // TODO: Animation for turning? At least avoiding wall?
                    let delta = self.field.map[*bot].dir + drift_dir;
                    if self.field.map.loc_at(*bot + delta).passable() {
                        self.field.map.move_delta(bot, delta);
                    }

                    // Hero dies if mov moves onto hero
                    if self.field.map[*bot].effect == Effect::Kill {
                        if bot.x == self.field.ros.hero.x && bot.y == self.field.ros.hero.y {
                            return Some(Continuation::PlayDie);
                        }
                    }
                },
                AI::Scuttle => {
                    // If hitting wall, choose new direction.
                    if self.field.map.loc_at(*bot + self.field.map[*bot].dir).impassable() {
                        let dx_to_hero = self.field.ros.hero.x - bot.x;
                        let dy_to_hero = self.field.ros.hero.y - bot.y;
                        // Find whether x or y is more towards the hero
                        let x_longer_than_y = match dx_to_hero.abs() - dy_to_hero.abs() {
                            num if num > 0 => true,
                            num if num < 0 => false,
                            _ => self.field.map[*bot].dir.dy.abs() < self.field.map[*bot].dir.dy.abs(),
                        };
                        // dlongcoord is the orthogonal direction most towards the hero. dshortcoord is the other best.
                        let (dlongcoord, dshortcoord) = if x_longer_than_y {
                            (CoordDelta::from_xy(dx_to_hero.signum(), 0), CoordDelta::from_xy(0, dy_to_hero.signum()))
                        } else {
                            (CoordDelta::from_xy(0, dy_to_hero.signum()), CoordDelta::from_xy(dx_to_hero.signum(), 0))
                        };
                        // Prefer the directions "most" towards the hero first
                        let try_dirs = vec![dlongcoord, dshortcoord, -dshortcoord, -dlongcoord];
                        // Try each direction in turn, use the first passable one.
                        // Can't be the same as original direction because that was impassable.
                        // If none are passable, stay in the same direction we started.
                        if let Some(dir) = try_dirs.iter().find(|dir|
                            self.field.map.loc_at(*bot + **dir).passable()
                        ) {
                            self.field.map[*bot].dir = *dir;
                        }
                    }

                    // Move. Provided next space is passable. If all sides were impassable, don't move.
                    if self.field.map.loc_at(*bot + self.field.map[*bot].dir).passable() {
                        self.field.map.move_delta(bot, self.field.map[*bot].dir);
                    }

                    // Hero dies if bot moves onto hero
                    if self.field.map[*bot].effect == Effect::Kill {
                        if bot.x == self.field.ros.hero.x && bot.y == self.field.ros.hero.y {
                            return Some(Continuation::PlayDie);
                        }
                    }
                },
            }
        }
        return None
    }

}
