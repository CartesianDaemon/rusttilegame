use macroquad::prelude::*;

mod input;
use input::*;
mod map;
use map::*;
mod load;
// use load::*;

// Might like types:
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
        // TODO: Should choice be in a Render function?
        match self.p.mode {
            Mode::LevPlay(_) => {
                let r = RenderLev::begin(self.p.map.w(), self.p.map.h());
                // Coords of first visible tile. Currently always 0,0.
                let (ox, oy) = (0, 0);
                for (x, y, loc) in self.p.map.locs() {
                    for ent in &loc.ents {
                        r.draw_ent(x - ox, y - oy, ent);
                    }
                }
            }
            Mode::NewGame | Mode::LevIntro(_) | Mode::Retry(_) => {
                let _r = RenderSplash::begin(&self.p.splash_text);
            }
            Mode::LevOutro(_) => {
                let _r = RenderSplash::begin(&self.p.outro_text);
            }
        }
    }
}


//
// PLAY
//
// TODO: Move Play to play submod?
//

// Whether we are currently playing a level, in intro screen, in game over, etc
//
// TODO: Make "State" in load which defines possible game states. Add "Mode" here
// which only has "Splash" vs "LevPlay". load() should take a state as argument
// and return a meaningful play, including Mode to render as, and win/loss States.
// Possibly adding a map Continuation -> next-state, for continuations "continue",
// "win", "loss".
//
// Currently hardcode that we go newgame -> levintro(1) -> levplay(1) -> levoutro(1)
// -> levintro(2) etc. And that game loss goes -> retry.
//
// Not sure if using level as part of enum is useful enough or not.
#[allow(dead_code)]
enum Mode {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    Retry(u16),
}

// Gameplay state: current level, map, etc.
struct Play {
    // Current mode, e.g. "New Game screen" or "Intro to level 1".
    mode: Mode,

    // Text for current interstitial screen. Levels use splash_txt
    // before and outro_text after.
    splash_text: String,
    outro_text: String,

    // TODO: Want to add a State struct like Mode but with meta info about level
    // like "next level". Which could be part of Play expected to be immutable.

    // Layout of current map, used in LevPlay.
    map: Map,
    ros: Ros,
}

impl Play {
    fn new_empty_level() -> Play {
        Play {
            mode: Mode::LevPlay(1),

            splash_text: "SPLASH TEXT".to_string(),
            outro_text: "OUTRO TEXT".to_string(),

            map: Map::new(16),
            ros: Ros::new(16),
        }
    }

    fn spawn_hero(&mut self, x: i16, y: i16, ent: Ent) {
        self.ros.hero = (x, y, 0);
        self.map.put_at(&mut self.ros.hero, ent);
    }

    fn spawn_mov(&mut self, x: i16, y: i16, ent: Ent) {
        let mut new_pos = (x, y, 0);
        self.map.put_at(&mut new_pos, ent);
        self.ros.push_mov(new_pos);
    }

    // Does current mode need UI to wait for tick before updating state?
    // Currently yes during play of level, no in splash screens.
    fn continuous(&self) -> bool {
        match self.mode {
            Mode::NewGame |
            Mode::Retry(_) |
            Mode::LevIntro(_) |
            Mode::LevOutro(_) => true,
            Mode::LevPlay(_) => false,
        }
    }

    // Advance game state according to current state
    fn advance(&mut self, input : &mut Input) {
        match self.mode {
            Mode::LevPlay(_) => {
                self.advance_level(input.consume_keypresses());
            }
            // TODO: Is there a clearer way to express splash screen progressions?
            Mode::NewGame => {
                self.advance_splash(input, Mode::LevIntro(1));
            }
            Mode::Retry(levno) => {
                // TODO: Skip intro when retrying. But needs to actually load level,
                // not just advance splash without changing map.
                self.advance_splash(input, Mode::LevIntro(levno));
            }
            Mode::LevIntro(levno) => {
                self.advance_splash(input, Mode::LevPlay(levno));
            }
            _ => {
                panic!("Advancing unhandled game state");
            }
        }
    }

    pub fn advance_level(&mut self, last_key_pressed: Option<KeyCode>) {
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

        // TODO: Change `snake` to `mov`
        for snake in &mut self.ros.movs {
            match self.map[*snake].ai {
                AI::Stay => {
                    // Do nothing
                },
                AI::Hero => {
                    // TODO
                },
                AI::Snake => {
                    // if snake on same row xor column as hero, change dir to face hero
                    if (snake.0 == self.ros.hero.0) != (snake.1 == self.ros.hero.1) {
                        let new_dir: Delta = ((self.ros.hero.0 - snake.0).signum(),(self.ros.hero.1 - snake.1).signum());
                        self.map[*snake].dir = new_dir;
                    }

                    // NOTE: When snake goes out of bounds is Pplaceholder for real win condition.
                    if !(0..self.map.w() as i16).contains(&(snake.0 + self.map[*snake].dir.0)) ||
                        !(0..self.map.h() as i16).contains(&(snake.1 + self.map[*snake].dir.1))
                    {
                        self.progress_win();
                        return; // Avoids double borrow. TODO: I think it's logical to bail out?
                    }
                    else
                    {
                        // move snake to new location
                        let dir = self.map[*snake].dir;
                        self.map.move_delta(snake, dir);
                    }

                    // Die if snake moves onto hero
                    if snake.0 == self.ros.hero.0 && snake.1 == self.ros.hero.1 {
                        self.progress_die();
                        return; // Avoids double borrow. TODO: I think it's logical to bail out?
                    }
                }
            }
        }
    }

    fn currlev(&self) -> u16 {
        match self.mode {
            Mode::LevIntro(levno) | Mode::LevPlay(levno) | Mode::LevOutro(levno) | Mode::Retry(levno) => levno,
            Mode::NewGame => panic!("currlev not applicable at new game screen"),
        }
    }

    fn nextlev(&self) -> u16 {
        self.currlev() + 1
    }

    fn progress_win(&mut self) {
        *self = load::load_level(self.nextlev());
    }

    fn progress_die(&mut self) {
        *self = load::load_retry(self.currlev());
    }

    // TODO: Any clearer if it returned a bool for "progress" instead of progress_to param?
    pub fn advance_splash(&mut self, input: &mut Input, progress_to_mode: Mode) {
        let key = input.consume_keypresses();

        // Reset "most recent tick" when leaving menu.
        // TODO: Avoid needing as a parameter. ie:
        // Need to move into input code. Maybe "when starting level"?
        // As part of some standard mode transition code?
        input.last_update = get_time();

        if Some(KeyCode::Enter) == key {
            // TODO: 
            match progress_to_mode {
                Mode::LevIntro(levno) => *self = load::load_level(levno),
                Mode::LevPlay(levno) => self.mode = Mode::LevPlay(levno), // Map already loaded // TODO: Not right?
                _ => panic!("Advance a splash screen to unknown mode"),
            }
        }
    }
}


//
// RENDER
//
// TODO: Move Render structs to render submod?
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
            // TODO: Why does this work with landscape orientation?
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

        // TODO: Multi-line text. Ideally with dialog pics etc.
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
