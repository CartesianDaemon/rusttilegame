use crate::*;

use play::Play;
use input::Input;

// Overall game state.
// FIXME: Does this need to exist or could it be folded into main.rs or play.rs?
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
        /* Can read_input be combined with wait_for_tick? */
        self.i.read_input();

        /* For non-continuous modes, typically gameplay rather than splash, wait for
         * next tick to advance game state.
         *
         * There should be a better way of expressing this logic between play and render.
         */
        if self.p.continuous() || self.i.ready_for_tick() {
            self.p.advance(&mut self.i);
        }

        render::draw_frame(&self.p);
    }
}
