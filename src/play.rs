// Nice to move macroquad dependencies out of play. Help test stand alone.
use macroquad::prelude::*;

use std::collections::HashMap;

use crate::*;

use input::Input;
use map::Map;
use map::Ros;
use obj::Obj;
use map_coords::*;
use levset::LevstageBase;

/// Different types of stage, e.g. "gameplay" vs "splash screen"
///
/// Better if Play was an enum of these possibiltiies.
#[derive(Clone, Debug)]
pub enum Mode {
    /// Splash message, any key to continue. E.g. New level, game over.
    Splash,
    /// Interactive map, the actual gameplay part of the game.
    LevPlay,
}

/// Gameplay state: current level, map, etc.
///
/// Public fields should only be needed by Render or produced by load, not
/// used elsewhere.
///
/// Better as enum of mode types inheriting a common trait is poss.
///
/// Stores id of next stage through opaque LevstageBase trait object. It was a pain to
/// get the trait object to work. Also consider using a fixed-size type for LevstageBase.
/// Also considered making Play templated on LevSet at compile time.
///
/// Eventually we'll probably need to store the current Levstage.
#[derive(Clone, Debug)]
pub struct Play {
    /// Mode of current state, either an interstitial splash screen or a level to play.
    pub mode: Mode,

    /// Next stage to go to after continue or win.
    pub to_stage: Box<dyn LevstageBase>,
    // Next stage to go to after death. Only used in LevPlay not Splash. Currently always retry.
    pub die_stage: Box<dyn LevstageBase>,

    // Text for current interstitial screen. Only in Splash.
    pub splash_text: String,

    // Layout of current map. Only in LevPlay.
    pub map: Map,
    // Entities on the current map. Only in LevPlay.
    pub ros: Ros,

}

impl Play {
    pub fn make_splash(txt: String, to_stage:  Box<dyn levset::LevstageBase>,) -> Play {
        Play {
            mode: Mode::Splash,
            splash_text: txt,
            to_stage,

            // Won't be used
            // TODO: Empty default without using biobot types.
             die_stage: Box::new(levset_biobot::BiobotStage::NewGame),

            // Won't be used
            map: Map::new(16),
            ros: Ros::new(),
        }
    }

    pub fn levplay_from_ascii(
        ascii_map: &[&str; 16],
        map_key: HashMap<char, Vec<Obj>>,
        to_stage: Box<dyn levset::LevstageBase>,
        die_stage: Box<dyn levset::LevstageBase>,
    ) -> Play {
        // TODO: Get size from strings. Assert equal to default 16 in meantime.
        let mut play = Play {
            mode : Mode::LevPlay,

            splash_text: "SPLASH TEXT".to_string(), // Won't be used

            map: Map::new(16),
            ros: Ros::new(),

            to_stage,
            die_stage,
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

    // Add ent to map, and if necessary to roster's hero pos or list of movs
    pub fn spawn_at(&mut self, x: i16, y: i16, orig_obj: Obj) {
        // FIXME: Cloning solely so that we can examine is_hero etc after.
        let hdl = self.map.place_obj_at(x, y, orig_obj);
        let placed_obj = &self.map[hdl];

        if placed_obj.is_hero() {
            self.ros.hero = hdl;
        } else if placed_obj.is_roster() {
            self.ros.push_mov(hdl);
        }

    }

    // Does current mode need UI to wait for tick before updating state?
    // Yes during play of level, no in splash screens.
    pub fn continuous(&self) -> bool {
        match self.mode {
            Mode::Splash => true,
            Mode::LevPlay => false,
        }
    }

    // Advance game state according to current state
    pub fn advance(&mut self, input : &mut Input) -> Option<Box<dyn LevstageBase>> {
        match self.mode {
            Mode::LevPlay => {
                self.advance_level(input.consume_keypresses())
            }
            Mode::Splash => {
                self.advance_splash(input)
            }
        }
    }

    fn advance_level(&mut self, last_key_pressed: Option<KeyCode>) -> Option<Box<dyn LevstageBase>>  {
        // Need all the properties used in Ent.
        // May move "can move" like logic into load, along with the assorted properties.
        // While keeping movement code coordinating between ents here.
        use obj::*;

        // FIXME: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // STUB: Maybe display char moving out of sync with enemy.

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
                if self.map.can_move(self.ros.hero, dir) {
                    self.map.move_delta(&mut self.ros.hero, dir);
                    // STUB: Check for win condition on ents other than the lowest one.
                    if self.map[MapHandle::from_xyh(self.ros.hero.x, self.ros.hero.y, 0)].effect == Effect::Win {
                        return self.next_win(); // Previously didn't return??
                    }
                }
            }
        }

        // Move all movs
        for mov in &mut self.ros.movs {
            match self.map[*mov].ai {
                AI::Stay => {
                    // Do nothing
                },
                AI::Hero => {
                    // Handled separately.
                },
                // STUB: When we see what mov movement logic are like, try to combine them into one fn.
                AI::Snake => {
                    // if mov on same row xor column as hero, change dir to face hero
                    if (mov.x == self.ros.hero.x) != (mov.y == self.ros.hero.y) {
                        let new_dir = CoordDelta::from_xy((self.ros.hero.x - mov.x).signum(),(self.ros.hero.y - mov.y).signum());
                        self.map[*mov].dir = new_dir;
                    }

                    // NOTE: When mov goes out of bounds is placeholder for real win condition.
                    if !(0..self.map.w() as i16).contains(&(mov.x + self.map[*mov].dir.dx)) ||
                        !(0..self.map.h() as i16).contains(&(mov.y + self.map[*mov].dir.dy))
                    {
                        return self.next_win();
                    }
                    else
                    {
                        // move mov to new location
                        // TODO: Have a "move_dir" fn.
                        let dir = self.map[*mov].dir;
                        self.map.move_delta(mov, dir);
                    }

                    // Die if mov moves onto hero
                    if mov.x == self.ros.hero.x && mov.y == self.ros.hero.y {
                        return self.next_die();
                    }
                },
                AI::Bounce => {
                    // TODO: Make a Map:: fn for "at pos + dir, or appropriate default if off map"

                    // If hitting wall, reverse direction.
                    if self.map.loc_at(*mov + self.map[*mov].dir).impassable() {
                        self.map[*mov].dir = CoordDelta::from_xy(-self.map[*mov].dir.dx, -self.map[*mov].dir.dy);
                    }

                    // Move. Provided next space is passable. If both sides are impassable, don't
                    // move.
                    if self.map.loc_at(*mov + self.map[*mov].dir).passable() {
                        self.map.move_delta(mov, self.map[*mov].dir);
                    }
                    // Die if mov moves onto hero
                    if self.map[*mov].effect == Effect::Kill {
                        if mov.x == self.ros.hero.x && mov.y == self.ros.hero.y {
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

    fn advance_splash(&mut self, input: &mut Input) -> Option<Box<dyn LevstageBase>> {
        let key = input.consume_keypresses();

        // Reset "most recent tick" when leaving menu.
        // FIXME: Avoid needing input as a parameter, move time update to input code.
        input.last_update = get_time();

        if key.is_some() {
            return self.next_continue();
        }

        return None
    }

    fn next_continue(&self) -> Option<Box<dyn LevstageBase>> {
        Some(self.to_stage.clone())
    }
}
