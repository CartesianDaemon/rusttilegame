use crate::*;

use play::Play;
use input::Input;

/// Overall game state. Handles transitions between different Plays for different levstates.
///
/// Templated on LevSet (either a  builtin LevSet, or a load-from-file LevSet). Also considered
/// taking a &dyn LevSet trait object so that it could be linked with compiled level sets but
/// ran into difficulties with trait objects. Specifically, implemnentations of LevSet each need
/// a specific Levstage type, but a dyn LevStage pointer can't have an unspecified associated type.
pub struct Game<Levs: load::LevSet> {
    pub lev_set: Levs, // TODO
    play: Play,
    input: Input,
}

impl<Levs: load::LevSet> Game<Levs> {
    pub fn new(lev_set: Levs) -> Game<Levs> {
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
