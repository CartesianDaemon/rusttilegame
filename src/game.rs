// TODO: Remove unused use

use crate::*;

use play::Play;
use input::Input;

// Coord types (in theory)
// 
// FIXME: Move to a coord type module.
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
pub type Pos = (i16, i16, u16);
pub type Point = (i16, i16);
pub type Delta = (i16, i16);

// Overall game state.
// TODO: Does this need to exist or could it be folded into main.rs or play.rs?
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

        render::draw_frame(&self.p);
    }
}


// FIXME: Move Play to play submod.
// Whether we are currently playing a level, in intro screen, in game over, etc
pub enum Mode {
    Splash,
    LevPlay,
}
