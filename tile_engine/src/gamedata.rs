//use crate::simple_custom_props;

/// Trait for interface needed for Games implemented in the Engine

use super::pane::{Pane, Arena, PaneConclusion, Split};

// TODO: Don't need to for the first two games, but can move Pass and
// Effect in here. Or better, make a SimpleObjectInteractions type
// which isn't required but different MovementLogic customisations can
// use.
// TODO: Could merge CustomProps into MovementLogic, as CustomLogic.
// Defn would be mostly props. Impl would be mostly logic.
// Is that where fns like "all(Passable)" live?
pub trait BaseCustomProps : Clone + std::fmt::Debug + PartialEq {
    fn default() -> Self;

    /// Identifies objects which the engine needs to have move themselves.
    fn is_any_mov(self: &Self) -> bool;

    /// Identifies objects which move half a step ahead of other movs.
    /// Currently engine assumes only one hero exists.
    fn is_hero(self: &Self) -> bool;
}

use super::pane_arena::RosterIndex;
use crate::for_gamedata::Cmd;
use super::pane::PaneContinuation;

// NB: Fns only applicable to some widgets. Should be in type related to those.
pub trait BaseMovementLogic : Sized {
    type CustomProps : BaseCustomProps;
    fn harmonise(_split: Split<Self>) {
    }
    fn move_mov(map: &mut Arena<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation;
}

/// Manages game-specific state, e.g. which level to go to next.
pub trait BaseGamedata {
    type CustomProps : BaseCustomProps;
    type MovementLogic : BaseMovementLogic;

    fn new() -> Self;

    fn advance_pane(&mut self, continuation: PaneConclusion);

    fn load_pane(&self) -> Pane<Self::MovementLogic>;

    fn load_next_pane(&mut self, continuation: PaneConclusion) -> Pane<Self::MovementLogic> {
        self.advance_pane(continuation);
        self.load_pane()
    }
}
