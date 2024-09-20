use crate::*;

use load::LevSet;
use play::Play;
use input::Input;

// Overall game state.
// FIXME: Does this need to exist or could it be folded into main.rs or play.rs?
pub struct Game {
    pub lev_set: biobot::BiobotLevSet, // TODO

    play: Play,
    input: Input,
}

impl Game {
    pub fn new_default() -> Game {
        let lev_set = biobot::BiobotLevSet {};
        let play = lev_set._load_lev_stage(lev_set.initial_lev_stage());
        Game {
            lev_set,
            play,
            input: Input::new_default(),
        }
    }

    pub fn do_frame(&mut self) {
        /* Can read_input be combined with wait_for_tick? */
        self.input.read_input();

        /* For non-continuous modes, typically gameplay rather than splash, wait for
         * next tick to advance game state.
         *
         * There should be a better way of expressing this logic between play and render.
         */
        if self.play.continuous() || self.input.ready_for_tick() {
            let next_opt = self.play.advance(&mut self.input);
            if let Some(next) = next_opt {
                self.play = self.lev_set.load_lev_stage(&next);
            }
        }

        render::draw_frame(&self.play);
    }
}
