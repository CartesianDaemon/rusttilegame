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
    play_state: Play,
    ghost_state: Play,
    input: Input,
}

impl<Levs: levset::LevSet> Game<Levs> {
    pub fn new(lev_set: Levs) -> Game<Levs> {
        let play = lev_set._load_lev_stage(lev_set.initial_lev_stage());
        Game {
            lev_set,
            ghost_state: play.clone(),
            play_state: play,
            input: Input::new_begin(),
        }
    }

    /// Collect input. Draw frame. Advance logical game state, if tick scheduled.
    pub fn do_frame(&mut self) {
        /* ENH: Can read_input be combined with wait_for_tick? */
        self.input.read_input();

        if self.play_state.continuous() || self.input.ready_to_advance_game_state() {
            let maybe_to_lev = self.play_state.advance(&mut self.input);
            if let Some(to_lev) = maybe_to_lev {
                self.play_state = self.lev_set.load_lev_stage(&to_lev);
            }
            self.ghost_state = self.play_state.clone();
            // self.ghost_state.advance(&mut self.input); // Even once get index out of bounds. Try in test.
        } else if self.input.ready_to_advance_ghost_state() {
            self.ghost_state.advance(&mut self.input);
        }

        render::draw_frame(&self.play_state, &self.ghost_state);
    }
}
