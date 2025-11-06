//use crate::engine::obj_scripting_properties;

use crate::engine::for_gamedata::BaseObjScriptProps;

/// Trait for interface needed for Games implemented in the Engine

use super::pane::{Pane, PaneEnding};

// Trait for scripts which the scripts for each game needs to implement.
// TODO: Move to separate file??
use super::map::DefaultMap as Map;
use super::map::RosterIndex;
use super::for_gamedata::Cmd;
use super::pane::PaneContinuation;
pub trait BaseMovementLogic {
    fn move_mov(field: &mut Map, mov: RosterIndex, cmd: Cmd) -> PaneContinuation;
}
pub trait BaseScripts {
    type MovementLogic : BaseMovementLogic;
}

/// Manages game-specific state, e.g. which level to go to next.
pub trait BaseGamedata {
    type ObjScriptProps : BaseObjScriptProps;
    type Scripts : BaseScripts;
    //type XAI;

    fn new() -> Self;

    fn advance_pane(&mut self, continuation: PaneEnding);

    fn load_pane(&self) -> Pane<Self::ObjScriptProps>;

    fn load_next_pane(&mut self, continuation: PaneEnding) -> Pane<Self::ObjScriptProps> {
        self.advance_pane(continuation);
        self.load_pane()
    }
}
