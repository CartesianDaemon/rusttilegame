use crate::*;

use play::Play;
use input::Input;

/// Overall game state. Handles transitions between different Plays for different levstates.
///
/// Templated on LevSet (either a  builtin LevSet, or a load-from-file LevSet).
///
/// Could also take a &dyn LevSet trait object so that it could be linked with compiled level
/// sets, but need to establish how to pass an appropriate LevStage pointer to the concrete
/// class.
pub struct Game<Levs: levset::LevSet> {
    pub lev_set: Levs, // TODO
    play: Play,
    input: Input,
}

impl<Levs: levset::LevSet> Game<Levs> {
    pub fn new(lev_set: Levs) -> Game<Levs> {
        let play = lev_set._load_lev_stage(lev_set.initial_lev_stage());
        Game {
            lev_set,
            play,
            input: Input::new_default(),
        }
    }

    /// Collect input. Draw frame. Advance logical game state, if tick scheduled.
    pub fn do_frame(&mut self) {
        /* ENH: Can read_input be combined with wait_for_tick? */
        self.input.read_input();

        if self.play.continuous() || self.input.ready_for_tick() {
            let next_opt = self.play.advance(&mut self.input);
            if let Some(next) = next_opt {
                self.play = self.lev_set.load_lev_stage(&next);
            }
        }

        render::draw_frame(&self.play);
    }
}
