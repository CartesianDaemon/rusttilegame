use std::collections::HashMap;

use macroquad::prelude::*;

use crate::*;

use input::Input;

// use crate::game::load;

use map::Map;
use map::Ros;
use ent::Ent;

use types::Delta;

use game::Mode;
use load::Stage;

// Gameplay state: current level, map, etc.
// STUB: Public fields should only be needed by Render or produced by load, not
// used elsewhere.
pub struct Play {
    // Current mode, e.g. "New Game screen" or "Intro to level 1".
    pub mode: Mode,

    // Text for current interstitial screen in Mode::Splash.
    pub splash_text: String,

    // Layout of current map, used in Mode::LevPlay.
    pub map: Map,
    pub ros: Ros,

    // Next stage to go to after continue or win.
    pub to_stage: Stage,
    // Next stage to go to after death. Currently always retry.
    pub die_stage: Stage,
}

impl Play {
    fn new_empty_level() -> Play {
        Play {
            // TODO: Add current Stage.
            mode: Mode::Splash, // Should always get overridden

            splash_text: "SPLASH TEXT".to_string(),

            map: Map::new(16),
            ros: Ros::new(),

            to_stage: Stage::NewGame,
            die_stage: Stage::NewGame, // Shouldn't be used?
        }
    }

    pub fn new_null_level() -> Play {
        Self::new_empty_level()
    }

    pub  fn from_ascii(ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
        // TODO: Get size from strings. Assert equal to default 16 in meantime.
        let mut play = Play::new_empty_level();

        for (y, line) in ascii_map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                for ent in map_key.get(&ch).unwrap() {
                    play.spawn_at(x as i16, y as i16, ent.clone());
                }
            }
        }

        play
    }

    /*
    // Pos for hero, which can be set to match newly added hero in map.
    // TODO: Where this logic should be centralised
    fn hero_pos_mut(&mut self) -> &mut Pos{
        &mut self.ros.hero
    }

    // Pos for new entry in mov roster, which can be set to match new mov in map.
    // TODO: Where this logic should be centralised
    fn new_mov_pos_mut(&mut self) -> &mut Pos{
        self.ros
        &mut self.ros.hero
    }
    */

    // Add ent to map, and if necessary to roster's hero pos or list of movs 
    pub fn spawn_at(&mut self, x: i16, y: i16, ent: Ent) {
        let mut pos = (x, y, 0);

        // FIXME: Cloning solely so that we can examine is_hero etc after.
        self.map.put_at(&mut pos, ent.clone()); // Sets height correctly

        if ent.is_hero() {
            self.ros.hero = pos;
        } else if ent.is_roster() {
            self.ros.push_mov(pos);
        }

    }

    // Does current mode need UI to wait for tick before updating state?
    // Currently yes during play of level, no in splash screens.
    // Simplified if we have game State and Play/Splash mode.
    pub fn continuous(&self) -> bool {
        match self.mode {
            Mode::Splash => true,
            Mode::LevPlay => false,
        }
    }

    // Advance game state according to current state
    pub fn advance(&mut self, input : &mut Input) {
        match self.mode {
            Mode::LevPlay => {
                self.advance_level(input.consume_keypresses());
            }
            Mode::Splash => {
                self.advance_splash(input, self.to_stage);
            }
        }
    }

    fn advance_level(&mut self, last_key_pressed: Option<KeyCode>) {
        // Need all the properties used in Ent.
        // May move properties and this function into load.
        use ent::*;

        // FIXME: Decide order of char, enemy. Before or after not quite right. Or need
        // to handle char moving onto enemy.
        // STUB: Maybe display char moving out of sync with enemy.

        // Move character
        if let Some(key) = last_key_pressed {
            let mut dir = (0, 0);
            match key {
                KeyCode::Left  => dir = (-1, 0),
                KeyCode::Right => dir = (1, 0),
                KeyCode::Up    => dir = (0, -1),
                KeyCode::Down  => dir = (0, 1),
                _ => (),
            }
            if dir != (0, 0) {
                if self.map.can_move(&self.ros.hero, dir) {
                    self.map.move_delta(&mut self.ros.hero, dir);
                    // STUB: Check for win condition on ents other than the lowest one.
                    if self.map[(self.ros.hero.0, self.ros.hero.1, 0)].effect == Effect::Win {
                        self.progress_win();
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
                    if (mov.0 == self.ros.hero.0) != (mov.1 == self.ros.hero.1) {
                        let new_dir: Delta = ((self.ros.hero.0 - mov.0).signum(),(self.ros.hero.1 - mov.1).signum());
                        self.map[*mov].dir = new_dir;
                    }

                    // NOTE: When mov goes out of bounds is placeholder for real win condition.
                    if !(0..self.map.w() as i16).contains(&(mov.0 + self.map[*mov].dir.0)) ||
                        !(0..self.map.h() as i16).contains(&(mov.1 + self.map[*mov].dir.1))
                    {
                        self.progress_win();
                        return; // NOTE: Bail out as more updates may not make sense. Necessary to avoid double borrow.
                    }
                    else
                    {
                        // move mov to new location
                        // TODO: Have a "move_dir" fn.
                        let dir = self.map[*mov].dir;
                        self.map.move_delta(mov, dir);
                    }

                    // Die if mov moves onto hero
                    if mov.0 == self.ros.hero.0 && mov.1 == self.ros.hero.1 {
                        self.progress_die();
                        return; // NOTE: Bail out as more updates may not make sense. Necessary to avoid double borrow.
                    }
                },
                AI::Bounce => {
                    // TODO: Make a Map:: fn for "at pos + dir, or appropriate default if off map"

                    // If hitting wall, reverse direction.
                    if self.map.loc_at((mov.0 + self.map[*mov].dir.0, mov.1 + self.map[*mov].dir.1, 0)).impassable() {
                        self.map[*mov].dir = (-self.map[*mov].dir.0, -self.map[*mov].dir.1);
                    }

                    // Move. Provided next space is passable. If both sides are impassable, don't
                    // move.
                    if self.map.loc_at((mov.0 + self.map[*mov].dir.0, mov.1 + self.map[*mov].dir.1, 0)).passable() {
                        self.map.move_delta(mov, self.map[*mov].dir);
                    }
                    // Die if mov moves onto hero
                    if self.map[*mov].effect == Effect::Kill {
                        if mov.0 == self.ros.hero.0 && mov.1 == self.ros.hero.1 {
                            self.progress_die();
                            return; // NOTE: Bail out as more updates may not make sense. Necessary to avoid double borrow.
                        }
                    }
                }
            }
        }
    }

    fn progress_win(&mut self) {
        *self = load::load_stage(self.to_stage);
    }

    fn progress_die(&mut self) {
        *self = load::load_stage(self.die_stage);
    }

    pub fn advance_splash(&mut self, input: &mut Input, progress_to_stage: Stage) {
        let key = input.consume_keypresses();

        // Reset "most recent tick" when leaving menu.
        // FIXME: Avoid needing input as a parameter, move time update to input code.
        input.last_update = get_time();

        if Some(KeyCode::Enter) == key {
            *self = load::load_stage(progress_to_stage);
        }
    }
}
