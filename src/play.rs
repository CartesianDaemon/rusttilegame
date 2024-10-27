// Nice to move macroquad dependencies out of play. Help test stand alone.
use macroquad::prelude::*;

use std::collections::HashMap;

use crate::*;

use input::Input;
use field::Field;
use obj::Obj;
use map_coords::*;
use levset::LevstageBase;

/// Interactive map, the actual gameplay part of the game.
#[derive(Clone, Debug)]
pub struct LevPlay {
    /// Next stage to go to after win.
    pub to_stage: Box<dyn LevstageBase>,
    // Next stage to go to after death. In levset_biobots always retry.
    pub die_stage: Box<dyn LevstageBase>,

    // Layout of current map.
    pub field: Field,
}

/// Splash message, any key to continue. E.g. New level, game over.
#[derive(Clone, Debug)]
pub struct Splash {
    /// Next stage to go to after continue.
    pub to_stage: Box<dyn LevstageBase>,

    // Text for current interstitial screen. Only in Splash.
    pub splash_text: String,
    pub dialogue: Dialogue, // If this works, will replace splash_text
}

/// Gameplay state: current level, map, etc.
///
/// Public fields should only be needed by Render or produced by load, not
/// used elsewhere.
///
/// Stores id of next stage through opaque LevstageBase trait object. It was a pain to
/// get the trait object to work. Also consider using a fixed-size type for LevstageBase.
/// Also considered making Play templated on LevSet at compile time.
///
/// Eventually we'll probably need to store the current Levstage.
#[derive(Clone, Debug)]
pub enum Play {
    LevPlay(LevPlay),
    Splash(Splash),
}

impl Play {
    pub fn make_splash(txt: String, to_stage:  Box<dyn levset::LevstageBase>,) -> Play {
        Play::Splash( Splash {
            splash_text: txt,
            dialogue: Dialogue { entries: vec![]},
            to_stage,
        })
    }

    pub fn make_dialogue(entries: Vec<&str>, to_stage:  Box<dyn levset::LevstageBase>,) -> Play {
        Play::Splash( Splash {
            splash_text: "".to_string(),
            dialogue: Dialogue { entries: entries.iter().map(|x| DialogueLine {tex_path: "".to_string(), text: x.to_string()} ).collect() },
            to_stage,
        })
    }

    // TODO: Move to LevPlay
    // TODO: Do we need a function or would having levset_biobots use LevPlay {...} be better?
    // TODO: Use lifetime or Rc on map_key instead of clone()?
    pub fn levplay_from_ascii(
        ascii_map: &[&str; 16],
        map_key: HashMap<char, Vec<Obj>>,
        to_stage: Box<dyn levset::LevstageBase>,
        die_stage: Box<dyn levset::LevstageBase>,
    ) -> Play {
        // TODO: Get size from strings. Assert equal to default 16 in meantime.
        let mut levplay = LevPlay {
            field: Field {
                map_key: map_key.clone(),
                ..Field::empty(16)
            },

            to_stage,
            die_stage,
        };

        for (y, line) in ascii_map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                for ent in map_key.get(&ch).unwrap() {
                    levplay.spawn_at(x as i16, y as i16, ent.clone());
                }
            }
        }

        Play::LevPlay(levplay)
    }

    // Does current mode need UI to wait for tick before updating state?
    // Yes during play of level, no in splash screens.
    pub fn continuous(&self) -> bool {
        match self {
            Self::Splash(_) => true,
            Self::LevPlay(_) => false,
        }
    }

    // Advance game state according to current state
    pub fn advance(&mut self, input : &mut Input) -> Option<Box<dyn LevstageBase>> {
        match self {
            Self::LevPlay(play) => play.advance(input.consume_keypresses()),
            Self::Splash(play) => play.advance(input),
        }
    }

    pub fn to_levplay_or_placeholder(&self) -> LevPlay {
        match self {
            Self::LevPlay(levplay) => levplay.clone(),
            Self::Splash(splash) => LevPlay {
                field: Field::empty(16),
                to_stage: splash.to_stage.clone(),
                die_stage: splash.to_stage.clone(),
            },
        }
    }
}

impl LevPlay
{
    /// Add ent to map.
    ///
    /// Might not be necessary as a separate fn now roster logic in field.
    pub fn spawn_at(&mut self, x: i16, y: i16, orig_obj: Obj) {
        self.field.place_obj_at(x, y, orig_obj);
    }

    pub fn advance(&mut self, last_key_pressed: Option<KeyCode>) -> Option<Box<dyn LevstageBase>>  {
        // Need all the properties used in Ent.
        // May move "can move" like logic into load, along with the assorted properties.
        // While keeping movement code coordinating between ents here.
        use obj::*;

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
                        return self.next_win(); // Previously didn't return??
                    }
                }
            }
        }

        // Move all movs
        for mov in &mut self.field.ros.movs {
            // Before movement, reset "prev". Will be overwritten if movement happens.
            self.field.map[*mov].prev_pos = self.field.map[*mov].cached_pos;

            match self.field.map[*mov].ai {
                AI::Stay => {
                    // Do nothing
                },
                AI::Hero => {
                    // Handled separately.
                },
                // STUB: When we see what mov movement logic are like, try to combine them into one fn.
                AI::Snake => {
                    // if mov on same row xor column as hero, change dir to face hero
                    if (mov.x == self.field.ros.hero.x) != (mov.y == self.field.ros.hero.y) {
                        let new_dir = CoordDelta::from_xy((self.field.ros.hero.x - mov.x).signum(),(self.field.ros.hero.y - mov.y).signum());
                        self.field.map[*mov].dir = new_dir;
                    }

                    // NOTE: When mov goes out of bounds is placeholder for real win condition.
                    if !(0..self.field.map.w() as i16).contains(&(mov.x + self.field.map[*mov].dir.dx)) ||
                        !(0..self.field.map.h() as i16).contains(&(mov.y + self.field.map[*mov].dir.dy))
                    {
                        return self.next_win();
                    }
                    else
                    {
                        // move mov to new location
                        // TODO: Have a "move_dir" fn.
                        let dir = self.field.map[*mov].dir;
                        self.field.map.move_delta(mov, dir);
                    }

                    // Die if mov moves onto hero
                    if mov.x == self.field.ros.hero.x && mov.y == self.field.ros.hero.y {
                        return self.next_die();
                    }
                },
                AI::Bounce => {
                    // TODO: Make a Map:: fn for "at pos + dir, or appropriate default if off map"

                    // If hitting wall, reverse direction.
                    if self.field.map.loc_at(*mov + self.field.map[*mov].dir).impassable() {
                        self.field.map[*mov].dir = CoordDelta::from_xy(-self.field.map[*mov].dir.dx, -self.field.map[*mov].dir.dy);
                    }

                    // Move. Provided next space is passable. If both sides are impassable, don't
                    // move.
                    if self.field.map.loc_at(*mov + self.field.map[*mov].dir).passable() {
                        self.field.map.move_delta(mov, self.field.map[*mov].dir);
                    }
                    // Die if mov moves onto hero
                    if self.field.map[*mov].effect == Effect::Kill {
                        if mov.x == self.field.ros.hero.x && mov.y == self.field.ros.hero.y {
                            return self.next_die();
                        }
                    }
                }
            }
        }
        return None
    }

    fn next_win(&self) -> Option<Box<dyn LevstageBase>> {
        Some(self.to_stage.clone())
    }

    fn next_die(&self) -> Option<Box<dyn LevstageBase>> {
        Some(self.die_stage.clone())
    }
}

impl Splash
{
    fn advance(&mut self, input: &mut Input) -> Option<Box<dyn LevstageBase>> {
        let key = input.consume_keypresses();

        // Reset "most recent tick" when leaving menu.
        // FIXME: Avoid needing input as a parameter, move time update to input code.
        input.last_real_update = get_time();

        if key.is_some() {
            return self.next_continue();
        }

        return None
    }

    fn next_continue(&self) -> Option<Box<dyn LevstageBase>> {
        Some(self.to_stage.clone())
    }
}

#[derive(Clone, Debug)]
pub struct DialogueLine {
    pub tex_path: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Dialogue {
    pub entries: Vec<DialogueLine>,
}