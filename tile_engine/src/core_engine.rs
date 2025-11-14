use crate::gamedata::BaseGamedata;

use super::gamedata;
use super::widget::*;
use super::input::Input;
use super::ui::UiBase;

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
    state: Widget<Gamedata::GameLogic>,

    /// Smoothly from 0 to 1 transition from previous state to current state
    /// TODO: Move into arena?
    /// TODO: Updated by input::ready_to_advance. Is that right? Could return tuple.
    /// TODO: Combine anim and slide..?
    anim_pc: f32,
    slide_pc: f32,

    /// Record input from user ready for use.
    input: Input,

    ///
    render: UiBase,
}

impl<Gamedata: gamedata::BaseGamedata> Engine<Gamedata> {
    pub fn new() -> Engine<Gamedata> {
        let gamedata = Gamedata::new();
        let arena = gamedata.load_pane();
        Engine::<Gamedata> {
            gamedata: gamedata,
            state: arena,
            anim_pc: 0.,
            slide_pc: 0.,
            input: Input::new_begin(),
            render: UiBase::new(),
        }
    }

    /// Collect input. Draw frame. Advance logical Engine state, if tick scheduled.
    pub async fn do_frame(&mut self) {
        self.input.read_input();

        // NB: Confusingly out of date! Currently we do a "tick" only when the user enters a key.
        // Tick never happens automatically. All tick length means is how long each moving
        // animation takes to complete
        if !self.state.tick_based() || self.input.ready_to_advance_game_state(&mut self.anim_pc, &mut self.slide_pc) {
            // Do a "tick". Actually, currently whenever user presses key.
            // TODO: Use Option<Cmd> not Cmd::default.
            let cmd = self.input.consume_cmd();
            let widget_continuation = self.state.advance(cmd);
            if let PaneContinuation::Break(widget_ending) = widget_continuation {
                self.state = self.gamedata.load_next_pane(widget_ending);
                self.input.last_tick_time = macroquad::prelude::get_time();
            }
        }

        self.render.draw_frame(
            &mut self.state,
            self.slide_pc,
            self.anim_pc,
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
