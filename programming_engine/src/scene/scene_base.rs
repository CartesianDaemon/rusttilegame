use std::ops::ControlFlow;

pub use super::arena::Arena;
pub use super::coding::*;
pub use super::splash::*;
pub use super::coding_arena::*;
pub use super::super::ui::InputCmd;
pub use crate::for_gamedata::OutcomeToStore;

use crate::for_gamedata;

// Determines which scene to go to after scene ends.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SceneConclusion {
    Continue,
    Succeed,
    Fail,
}

// After each tick, either Continue, or restart/start another level based on Conclusion.
// TODO: Move into Arena, only used there.
pub type SceneContinuation = ControlFlow<SceneConclusion, ()>;

pub trait BaseScene {
    fn advance(&mut self, cmd: InputCmd);
    fn ready_for_next_level(&self) -> Option<SceneConclusion>;
}

/// One unit of gameplay: one map layout, one splash screen, etc.
///
/// Often but not always one UI is a way to draw one corresponding scene.
///
/// Could make game-specific state more modularly include which scenes it
/// wants to use.
#[derive(Clone, Debug)]
pub enum Scene<MovementLogic: for_gamedata::BaseMovementLogic> {
    Splash(Splash),
    CodingArena(CodingArena<MovementLogic>),
    // Could be defined but not used separately:
    // Arena(Arena<MovementLogic>),
    //  Code(Code)
}

impl<MovementLogic: for_gamedata::BaseMovementLogic> Scene<MovementLogic> {
    pub fn from_splash_string(txt: String) -> Self {
        Scene::Splash(Splash::from_string(txt))
    }

    pub fn from_splash_dialogue(entries: Vec<&str>) -> Self {
        Scene::Splash(Splash::from_dialogue(entries))
    }

    // Advance game state. Called when clock ticks or when user inputs.
    pub fn advance(&mut self, cmd: InputCmd) {
        // NB: Use the crate that makes it easy to inherit behaviour between enum variants.
        match self {
            Self::Splash(scene) => scene.advance(cmd),
            Self::CodingArena(scene) => scene.advance(cmd),
        }
    }

    pub fn ready_for_next_level(&self) -> Option<SceneConclusion> {
        match self {
            Self::Splash(scene) => scene.ready_for_next_level(),
            Self::CodingArena(scene) => scene.ready_for_next_level(),
        }
    }

    pub fn consume_outcome_to_store(&mut self) -> Option<OutcomeToStore> {
        match self {
            Self::Splash(_splash) => None,
            Self::CodingArena(scene) => scene.consume_outcome_to_store(),
        }
    }

    pub fn as_arena(&self) -> &Arena<MovementLogic> {
        match self {
            Self::Splash(_splash) => panic!(),
            Self::CodingArena(scene) => &scene.init_arena,
        }
    }

    // Used for debugging. Ideally would avoid .as_play().
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_arena().as_ascii_rows()
    }
}
