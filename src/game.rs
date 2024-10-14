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
    ghost_counter: GhostCounter,
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
            ghost_counter: GhostCounter {
                n_ghost_ticks: 0,
                config: GhostConfig {
                    preghost_ticks: 6,
                    max_ghost_ticks: 6,
                    tween_ghost_ticks: 0,
                    min_ghost_pc: 0.6,
                    max_ghost_pc: 0.9,
                }
            }
        }
    }

    fn init_ghost_state(&mut self) {
        self.ghost_state = self.play_state.clone();
        self.ghost_counter.n_ghost_ticks = self.ghost_counter.init_n_ticks();
    }

    fn reinit_ghost_state(&mut self) {
        self.ghost_state = self.play_state.clone();
        self.ghost_counter.n_ghost_ticks = self.ghost_counter.reinit_n_ticks();
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
            self.init_ghost_state();
        } else if self.input.ready_to_advance_ghost_state() {
            self.ghost_counter.n_ghost_ticks += 1;
            if self.ghost_counter.ready_to_reinit() {
                self.reinit_ghost_state();
            } else if self.ghost_counter.ready_to_advance_ghost_state() {
                self.ghost_state.advance(&mut self.input);
            }
        }

        render::draw_frame(
            &self.play_state,
            &self.ghost_state,
            self.ghost_counter.ghost_opacity(),
        );
    }
}

struct GhostConfig
{
    preghost_ticks: i32,
    max_ghost_ticks: i32,
    tween_ghost_ticks: i32,
    min_ghost_pc: f32,
    max_ghost_pc: f32,
}

struct GhostCounter
{
    n_ghost_ticks: i32, // Move into play state?
    config: GhostConfig,
}

impl GhostCounter
{
    pub fn init_n_ticks(&self) -> i32
    {
        0
    }

    pub fn reinit_n_ticks(&self) -> i32
    {
        self.config.preghost_ticks - self.config.tween_ghost_ticks
    }

    pub fn ready_to_reinit(&self) -> bool
    {
        self.n_ghost_ticks > self.config.preghost_ticks + self.config.max_ghost_ticks
    }

    pub fn ready_to_advance_ghost_state(&self) -> bool
    {
        self.n_ghost_ticks >= self.config.preghost_ticks
    }

    fn ghost_opacity(&self) -> f32 {
        if self.n_ghost_ticks < self.config.preghost_ticks {
            0.5 // Either 0 or 1 should be equally good?
        } else {
            self.config.min_ghost_pc +
            (self.config.max_ghost_pc - self.config.min_ghost_pc) *
            (self.n_ghost_ticks - self.config.preghost_ticks) as f32 / self.config.max_ghost_ticks as f32
        }
    }

}