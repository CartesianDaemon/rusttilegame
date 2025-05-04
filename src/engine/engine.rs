use super::*;

// TODO: Remove submodule names?
use super::scene::Scene;
use super::input::Input;
use super::render::Render;
use super::scene::*;

/// Overall Engine state.
///
/// Including set of levels in current Engine, and state of current level being played.
///
/// Templated on Game (either a  builtin Game, or a load-from-file Game).
/// Could instead take a &dyn Game trait object so that it could load a Game object
/// from a library, but that probably doesn't help that much.
pub struct Engine<Game: gametrait::GameTrait> {
    /// Level set currently playing through, e.g. the biobot Engine.
    pub game: Game,

    /// Current state of gameplay, current level, mostly map etc.
    play_state: Scene,

    /// Smoothly from 0 to 1 transition from previous state to current state
    /// TODO: Move into play?
    /// TODO: Updated by input::ready_to_advance. Is that right? Could return tuple.
    /// TODO: Combine anim and slide..?
    anim_real_pc: f32,
    slide_real_pc: f32,

    /// Record input from user ready for use.
    input: Input,

    ///
    render: Render,
}

impl<Game: gametrait::GameTrait> Engine<Game> {
    pub fn new() -> Engine<Game> {
        let game = Game::new_game();
        let play = game.load_scene();
        Engine {
            game,
            play_state: play,
            anim_real_pc: 0.,
            slide_real_pc: 0.,
            input: Input::new_begin(),
            render: Render::new(),
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    pub async fn do_frame(&mut self) {
        /* ENH: Can read_input be combined with wait_for_tick? */
        self.input.read_input();

        if self.play_state.is_continuous() || self.input.ready_to_advance_game_state(&mut self.anim_real_pc, &mut self.slide_real_pc) {
            let scene_continuation = self.play_state.advance(&mut self.input);
            if let SceneContinuation::Break(scene_ending) = scene_continuation {
                self.play_state = self.game.load_next_scene(scene_ending);
            }
        }

        self.render.draw_frame(
            &self.play_state,
            self.slide_real_pc,
            self.anim_real_pc,
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
