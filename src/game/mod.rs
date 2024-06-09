// FIXME: Work out which types should be exported and remove use ::*.
// STUB: Move to separate modules not submodules with visibility.
mod play;
use play::*;

mod input;
use input::*;

mod render;
use render::RenderLev;
use render::RenderSplash;

mod map;
use map::*;

mod load;
use load::Stage;

mod util;

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

        // STUB: Would it be easier to read with a layout like:
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
        // FIXME: Choice of Render class should be made by a Render fn.
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


// FIXME: Move Play to play submod.
// Whether we are currently playing a level, in intro screen, in game over, etc
enum Mode {
    Splash,
    LevPlay,
}
