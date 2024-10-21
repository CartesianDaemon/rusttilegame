use crate::*;

use play::Play;
use input::Input;
use render::Render;

/// Overall game state. Handles transitions between different Plays for different levstates.
///
/// Templated on LevSet (either a  builtin LevSet, or a load-from-file LevSet).
///
/// Could also take a &dyn LevSet trait object so that it could be linked with compiled level
/// sets, but need to establish how to pass an appropriate LevStage pointer to the concrete
/// class.
pub struct Game<Levs: levset::LevSet> {
    /// Level set currently playing through, e.g. the biobot game.
    pub lev_set: Levs,

    /// Current state of gameplay, current level, mostly map etc.
    play_state: Play,

    /// Smoothly from 0 to 1 transition from previous state to current state
    /// TODO: Move into play?
    /// TODO: Updated by input::ready_to_advance. Is that right? Could return tuple.
    /// TODO: Combine anim and slide..?
    anim_real_pc: f32,
    slide_real_pc: f32,
    anim_ghost_pc: f32,

    /// Ghost state. Used to show where enemies are going to move
    /// TODO: Encapsulate better, or remove if not using.. Or fold into AnimState.
    ghost_state: play::LevPlay,
    ghost_counter: GhostCounter,

    /// Record input from user ready for use.
    input: Input,

    ///
    render: Render,
}

impl<Levs: levset::LevSet> Game<Levs> {
    pub fn new(lev_set: Levs) -> Game<Levs> {
        let play = lev_set._load_lev_stage(lev_set.initial_lev_stage());
        Game {
            lev_set,
            ghost_state: if let Play::LevPlay(levplay) = play.clone() {levplay} else { panic!() },
            play_state: play,
            anim_real_pc: 0.,
            slide_real_pc: 0.,
            anim_ghost_pc: 0.,
            input: Input::new_begin(),
            render: Render::new(),
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
        self.ghost_state = if let Play::LevPlay(levplay) = self.play_state.clone() {levplay} else { panic!() };
        self.ghost_counter.n_ghost_ticks = self.ghost_counter.init_n_ticks();
    }

    fn reinit_ghost_state(&mut self) {
        self.ghost_state = if let Play::LevPlay(levplay) = self.play_state.clone() {levplay} else { panic!() };
        self.ghost_counter.n_ghost_ticks = self.ghost_counter.reinit_n_ticks();
    }

    /// Collect input. Draw frame. Advance logical game state, if tick scheduled.
    pub async fn do_frame(&mut self) {
        /* ENH: Can read_input be combined with wait_for_tick? */
        self.input.read_input();

        if self.play_state.continuous() || self.input.ready_to_advance_game_state(&mut self.anim_real_pc, &mut self.slide_real_pc) {
            let maybe_to_lev = self.play_state.advance(&mut self.input);
            if let Some(to_lev) = maybe_to_lev {
                self.play_state = self.lev_set.load_lev_stage(&to_lev);
            }
            self.init_ghost_state();
        } else if self.input.ready_to_advance_ghost_state(&mut self.anim_ghost_pc) {
            self.ghost_counter.n_ghost_ticks += 1;
            if self.ghost_counter.ready_to_reinit() {
                self.reinit_ghost_state();
            } else if self.ghost_counter.ready_to_advance_ghost_state() {
                // TODO: Better abstraction
                self.ghost_state.advance(self.input.consume_keypresses());
            }
        }

        self.render.draw_frame(
            &self.play_state,
            self.slide_real_pc,
            self.anim_real_pc,
            &self.ghost_state,
            self.ghost_counter.ghost_opacity(),
            self.anim_ghost_pc,
        ).await;
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
            1. // Ghost obj should be in the same position as real obj, but not with smooth movement.
        } else {
            self.config.min_ghost_pc +
            (self.config.max_ghost_pc - self.config.min_ghost_pc) *
            (self.n_ghost_ticks - self.config.preghost_ticks) as f32 / self.config.max_ghost_ticks as f32
        }
    }

}
