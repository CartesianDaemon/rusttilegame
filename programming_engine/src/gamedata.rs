//use crate::simple_custom_props;

/// Trait for interface needed for Games implemented in the Engine

use super::scene::{Scene, Arena, SceneConclusion, CodingArena};

// TODO: Don't need to for the first two games, but can move Pass and
// Effect in here. Or better, make a SimpleObjectInteractions type
// which isn't required but different GameLogic customisations can
// use.
// TODO: Could merge CustomProps into GameLogic, as CustomLogic.
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

use super::scene::arena::RosterIndex;
use crate::for_gamedata::InputCmd;
use super::scene::SceneContinuation;

// NB: Fns only applicable to some scenes. Should be in type related to those.
pub trait BaseGameLogic : Clone + Sized {
    // For games with an Arena, game-specific data stored in each obj.
    type CustomProps : BaseCustomProps;

    // For games with an Arena, the logic for moving a movable obj.
    fn move_mov(map: &mut Arena<Self>, mov: RosterIndex, cmd: InputCmd) -> SceneContinuation;

    // For games with a CodingArena, coordinate the Arena with the Coding on advance.
    fn harmonise(_coding_arena: &mut CodingArena<Self>) {
    }

    // For games with a CodingArena, get which instr if any is currently executing.
    fn get_active_idx(_coding_arena: &CodingArena<Self>) -> Option<usize> {
        None
    }
}

/// Manages game-specific state, e.g. which level to go to next.
pub trait BaseGamedata {
    type CustomProps : BaseCustomProps;
    type GameLogic : BaseGameLogic;

    fn new() -> Self;

    fn advance_pane(&mut self, continuation: SceneConclusion);

    fn load_scene(&self) -> Scene<Self::GameLogic>;

    fn load_next_pane(&mut self, continuation: SceneConclusion) -> Scene<Self::GameLogic> {
        self.advance_pane(continuation);
        self.load_scene()
    }

    // Get string to display current level, in games with Arena or CodingArena.
    fn get_level_str(&self) -> String {
        String::new()
    }
}
