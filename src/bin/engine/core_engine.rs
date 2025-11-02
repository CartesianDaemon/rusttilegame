use super::*;

// TODO: Remove submodule names?
use super::scene::Scene;
use super::input::Input;
use super::render::Render;
use super::scene::*;

// Trait for scripts which the scripts for each game needs to implement.
// TODO: Move to separate file??
// TODO: Clone unneeded if we only template impl not struct
use super::field::Map;
use super::field::RosterIndex;
use super::for_gamedata::Cmd;
pub trait BaseMovementLogic {
    fn move_mov(field: &mut Map, mov: RosterIndex, cmd: Cmd) -> SceneContinuation;
}
pub trait BaseScripts {
    type MovementLogic : BaseMovementLogic;
}

/// Overall Engine state.
///
/// Including set of levels in current Engine, and state of current level being played.
///
/// Templated on Game (either a  builtin Game, or a load-from-file Game).
/// Could instead take a &dyn Game trait object so that it could load a Game object
/// from a library, but that probably doesn't help that much.
struct Engine<Game: base_gamedata::BaseGamedata> {
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

impl<Game: base_gamedata::BaseGamedata> Engine<Game> {
    pub fn new() -> Engine<Game> {
        let game = Game::new_game();
        let play = game.load_scene();
        Engine::<Game> {
            game,
            play_state: play,
            anim_real_pc: 0.,
            slide_real_pc: 0.,
            input: Input::new_begin(),
            render: Render::new(),
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    pub async fn do_frame<Scripts: super::for_scripting::BaseScripts>(&mut self) {
        /* ENH: Can read_input be combined with wait_for_tick? */
        self.input.read_input();

        if self.play_state.is_continuous() || self.input.ready_to_advance_game_state(&mut self.anim_real_pc, &mut self.slide_real_pc) {
            let scene_continuation = self.play_state.advance::<Scripts>(&mut self.input);
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

pub async fn run<Game: base_gamedata::BaseGamedata, Scripts: for_scripting::BaseScripts>()
{
    let mut engine = Engine::<Game>::new();

    loop {
        engine.do_frame::<Scripts>().await;
        macroquad::prelude::next_frame().await;
    }
}
