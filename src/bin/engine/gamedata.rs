//use crate::engine::simple_custom_props;

/// Trait for interface needed for Games implemented in the Engine

use super::pane::{Pane, PaneEnding};

pub trait BaseAI : Copy + PartialEq + std::fmt::Debug {
    /// Used to create default LogicalProps.
    /// Might not be needed if more logic moves into Gamedata.
    fn default() -> Self;
    fn is_hero(ai: Self) -> bool;
    fn is_any_mov(ai: Self) -> bool;
}

pub trait BaseCustomProps : Clone + std::fmt::Debug + PartialEq {
    fn default() -> Self;
}

use super::map::Map;
use super::map::RosterIndex;
use super::for_gamedata::Cmd;
use super::pane::PaneContinuation;
pub trait BaseMovementLogic : Sized + PartialEq {
    type CustomProps : BaseCustomProps;
    type AI : BaseAI;
    fn move_mov(map: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation;
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
