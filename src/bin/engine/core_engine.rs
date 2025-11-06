use crate::engine::gamedata::BaseGamedata;

use super::*;

// TODO: Remove submodule names?
use super::pane::Pane;
use super::input::Input;
use super::render::Render;
use super::pane::*;

/// Overall Engine state.
///
/// Including set of levels in current Engine, and state of current level being played.
///
/// Templated on Game (either a  builtin Game, or a load-from-file Game).
/// Could instead take a &dyn Game trait object so that it could load a Game object
/// from a library, but that probably doesn't help that much.
struct Engine<Gamedata: BaseGamedata> {
    /// Level set currently playing through, e.g. the biobot Engine.
    pub gamedata: Gamedata,

    /// Current state of gameplay, current level, mostly map etc.
    play_state: Pane<Gamedata::MovementLogic>,

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

impl<Gamedata: gamedata::BaseGamedata> Engine<Gamedata> {
    pub fn new() -> Engine<Gamedata> {
        let gamedata = Gamedata::new();
        let play = gamedata.load_pane();
        Engine::<Gamedata> {
            gamedata: gamedata,
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
            let pane_continuation = self.play_state.advance(&mut self.input);
            if let PaneContinuation::Break(pane_ending) = pane_continuation {
                self.play_state = self.gamedata.load_next_pane(pane_ending);
            }
        }

        self.render.draw_frame(
            &self.play_state,
            self.slide_real_pc,
            self.anim_real_pc,
        ).await;
    }
}

pub async fn run<Gamedata: gamedata::BaseGamedata>()
{
    let mut engine = Engine::<Gamedata>::new();

    loop {
        engine.do_frame().await;
        macroquad::prelude::next_frame().await;
    }
}
