//use crate::engine::simple_custom_props;

use crate::engine::for_gamedata::BaseCustomProps;

/// Trait for interface needed for Games implemented in the Engine

use super::pane::{Pane, PaneEnding};

// Trait for scripts which the scripts for each game needs to implement.
// TODO: Move to separate file??
use super::map::Map;
use super::map::RosterIndex;
use super::for_gamedata::Cmd;
use super::pane::PaneContinuation;
pub trait BaseMovementLogic : Sized {
    type CustomProps : BaseCustomProps;
    fn move_mov(field: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation;
}

/// Manages game-specific state, e.g. which level to go to next.
pub trait BaseGamedata {
    type CustomProps : BaseCustomProps;
    type MovementLogic : BaseMovementLogic;
    //type XAI;

    fn new() -> Self;

    fn advance_pane(&mut self, continuation: PaneEnding);

    fn load_pane(&self) -> Pane<Self::MovementLogic>;

    fn load_next_pane(&mut self, continuation: PaneEnding) -> Pane<Self::MovementLogic> {
        self.advance_pane(continuation);
        self.load_pane()
    }
}
