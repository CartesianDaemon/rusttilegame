use macroquad::prelude::*;

use std::collections::HashMap;

// FIXME: Work out which types should be exported and remove use ::*.
// FIXME: Move to separate modules not submodules with visibility.
mod input;
use input::*;
mod map;
use map::*;
mod load;
// use load::*;

// Coord types (in theory)
// 
// FIXME: Move to Map, or separate coord module.
// FIXME: Decide whether implementing types would help.
//
// Dimension: Width/height of map. Unsigned. Vars w,h.
// MapCoord: Coords on map. Signed to allow looping past edge.
//           May need cast to index vector? Vars x,y.
// ViewCoord: As MapCoord but relative to visible part of map (currently all).
//            Vars vx, vy.
// Delta: Offset of map coord. Signed. Vars dx, dy.
// PixCoord: Coords on screen. f32. Vars px, py.
// Pos: Coords including height.
//
// Ideally allowing arithmetic between dimension, map, delta with least casting.
// And multiplication of p coords by map coords.

// Coord types defined approximate theoretical types:
type Pos = (i16, i16, u16);
type Point = (i16, i16);
type Delta = (i16, i16);

// Overall game state.
pub struct Game {
    p: Play,
    i: Input,
}

impl Game {
    pub fn new_default() -> Game {
        Game {
            p: load::load_newgame(),
            i: Input::new_default(),
        }
    }

    pub fn do_frame(&mut self) {
        // Can I make a single function for this and ready_for_tick()?
        self.i.read_input();

        // Would it be easier to read with a layout like:
        //
        // while (not ready for tick) {
        //     accumulate_input();
        //     draw_frame();
        // }
        //
        // advance();
        //
        // draw_frame();
        //
        // ?
        //
        // But probably needs yield which we don't actually have?

        // Wait for tick if needed.
        // Need to know at this level to treat input differently on a tick
        // But maybe ready_for_tick can take a "tick wanted" parameter from Play mode.
        if self.p.continuous() || self.i.ready_for_tick() {
            self.p.advance(&mut self.i);
        }

        self.draw_frame();
    }

    fn draw_frame(&self) {
        // FIXME: Should choice of Render class be made by a Render fn?
        match self.p.mode {
            Mode::LevPlay => {
                let r = RenderLev::begin(self.p.map.w(), self.p.map.h());
                // Coords of first visible tile. Currently always 0,0.
                let (ox, oy) = (0, 0);
                for (x, y, loc) in self.p.map.locs() {
                    for ent in &loc.ents {
                        r.draw_ent(x - ox, y - oy, ent);
                    }
                }
            }
            Mode::Splash => {
                let _r = RenderSplash::begin(&self.p.splash_text);
            }
        }
    }
}


//
// PLAY
//
// FIXME: Move Play to play submod.
//

// Whether we are currently playing a level, in intro screen, in game over, etc
enum Mode {
    Splash,
    LevPlay,
}

// Gameplay state: current level, map, etc.
struct Play {
    // Current mode, e.g. "New Game screen" or "Intro to level 1".
    mode: Mode,

    // Text for current interstitial screen. Levels use splash_txt
    // before and outro_text after.
    splash_text: String,
    outro_text: String,

    // Layout of current map, used in LevPlay.
    map: Map,
    ros: Ros,

    // Next stage to go to after continue or win.
    to_stage: Stage,
    // Next stage to go to after death. Currently always retry.
    die_stage: Stage,
}

impl Play {
    fn new_empty_level() -> Play {
        Play {
            // TODO: Add current Stage.
            mode: Mode::Splash, // Should always get overridden

            splash_text: "SPLASH TEXT".to_string(),
            outro_text: "OUTRO TEXT".to_string(),

            map: Map::new(16),
            ros: Ros::new(),

            to_stage: Stage::NewGame,
            to_stage: Stage::NewGame, // Shouldn't be used?
        }
    }

    fn from_ascii(ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
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
    fn spawn_at(&mut self, x: i16, y: i16, ent: Ent) {
        let mut pos = (x, y, 0);

        // FIXME: Cloning solely so that we can examine is_hero etc after.
        self.map.put_at(&mut pos, ent.clone()); // Sets height correctly

        if is_hero(&ent) {
            self.ros.hero = pos;
        } else if is_roster(&ent) {
            self.ros.push_mov(pos);
        }

    }

    // Does current mode need UI to wait for tick before updating state?
    // Currently yes during play of level, no in splash screens.
    // Simplified if we have game State and Play/Splash mode.
    fn continuous(&self) -> bool {
        match self.mode {
            Mode::Splash => true,
            Mode::LevPlay => false,
        }
    }

    // Advance game state according to current state
    fn advance(&mut self, input : &mut Input) {
        match self.mode {
            Mode::LevPlay => {
                self.advance_level(input.consume_keypresses());
            }
            Mode::Splash => {
                self.advance_splash(input, self.to_stage);
            }
        }
    }

    pub fn advance_level(&mut self, last_key_pressed: Option<KeyCode>) {
        // FIXME: Decide order of char, enemy. Maybe even move out of sync.

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
                        let dir = self.map[*mov].dir;
                        self.map.move_delta(mov, dir);
                    }

                    // Die if mov moves onto hero
                    if mov.0 == self.ros.hero.0 && mov.1 == self.ros.hero.1 {
                        self.progress_die();
                        return; // NOTE: Bail out as more updates may not make sense. Necessary to avoid double borrow.
                    }
                }
            }
        }
    }

    fn nextlev(&self) -> u16 {
        self.currlev() + 1
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


//
// RENDER
//
// FIXME: Move Render structs to render submod?
//

// Render state for one frame of level
// Currently not needing any global graphics state
struct RenderLev {
    // COORDS FOR CURRENT FRAME. In gl units which are pixels.
    // Distance from edge of drawing surface to play area
    offset_x: f32,
    // Distance from edge of drawing surface to play area
    offset_y: f32,
    // Size of each tile
    sq_w: f32,
    sq_h: f32,
}

impl RenderLev {
    fn begin(w: u16, h: u16) -> RenderLev {
        assert_eq!(w, h);
        let game_size = screen_width().min(screen_height());
        let offset_y = (screen_height() - game_size) / 2. + 10.;

        let r = RenderLev {
            // FIXME: Why does this work with landscape orientation?
            offset_x: (screen_width() - game_size) / 2. + 10.,
            offset_y: (screen_height() - game_size) / 2. + 10.,
            sq_w: (screen_height() - offset_y * 2.) / w as f32,
            sq_h: (screen_height() - offset_y * 2.) / w as f32,
        };

        r._draw_backdrop();

        r
    }

    fn _draw_backdrop(&self)
    {
        clear_background(LIGHTGRAY);

        draw_text(format!("Level: 1", ).as_str(), 10., 20., 20., DARKGRAY);
    }

    // Draw ent's texture/colour to the screen at specified tile coords.
    // Works out pixel coords given pixel size of play area in RenderLev.
    fn draw_ent(
        self: &RenderLev,
        // View coords in map. Relative to first visible tile (currently always the same).
        vx: i16,
        vy: i16,
        // Ent to draw
        ent: &Ent,
    ) {
       let px = self.offset_x + self.sq_w * vx as f32;
       let py = self.offset_y + self.sq_h * vy as f32;

        if let Some(col) = ent.fill {
            draw_rectangle(px, py, self.sq_w, self.sq_h, col);
        }

        if let Some(col) = ent.border {
            draw_rectangle_lines(px, py, self.sq_w, self.sq_h, 2., col);
        }

        if let Some(tex) = &ent.tex {
            draw_texture_ex(
                &tex,
                px,
                py,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.sq_w, self.sq_h)),
                    ..Default::default()
                },
            );
        }
    }
}

// Render state for one frame of "Show text, press enter to continue"
// Currently not needing any global graphics state
struct RenderSplash {
}

impl RenderSplash
{
    fn begin(text: &str) -> RenderSplash {
        clear_background(WHITE);
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        // FIXME: Multi-line text. Ideally with dialog pics etc.
        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );

        RenderSplash {}
    }
}
