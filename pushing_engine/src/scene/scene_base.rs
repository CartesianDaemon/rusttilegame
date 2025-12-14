use std::collections::HashMap;
use std::ops::ControlFlow;

pub use super::arena::Arena;
pub use super::coding::*;
pub use super::splash::*;
pub use super::coding_arena::*;

use crate::map_coords::MoveCmd;
use crate::obj::FreeObj;
use crate::for_gamedata;

// How scene ended, used to determine next scene/lev to go to.
//
// Could be more game-specific.
#[derive(Debug, PartialEq)]
pub enum SceneConclusion {
    SplashContinue,
    Win,
    Die,
}

// After each tick, either Continue, or restart/start another level based on Conclusion.
pub type SceneContinuation = ControlFlow<SceneConclusion, ()>;

// NB Breadcrumb: Need different name for Scene ("level part") than Scene ("screen part").
pub trait BaseScene {
    fn tick_based(&self) -> crate::ui::TickStyle;
    fn advance(&mut self, cmd: MoveCmd) -> SceneContinuation;
}

/// One unit of gameplay: one map layout, one splash screen, etc.
///
/// Often but not always one UI is a way to draw one corresponding scene.
///
/// Could make game-specific state more modularly include which scenes it
/// wants to use.
#[derive(Clone, Debug)]
pub enum Scene<MovementLogic: for_gamedata::BaseMovementLogic> {
    Arena(Arena<MovementLogic>),
    Splash(Splash),
    CodingArena(CodingArena<MovementLogic>),
    // Could be defined but not used separately:
    //  Code(Code)
}

impl<MovementLogic: for_gamedata::BaseMovementLogic> Scene<MovementLogic> {
    pub fn from_splash_string(txt: String) -> Self {
        Scene::Splash(Splash::from_string(txt))
    }

    pub fn from_splash_dialogue(entries: Vec<&str>) -> Self {
        Scene::Splash(Splash::from_dialogue(entries))
    }

    pub fn from_play_ascii_map<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
    ) -> Self {
        Scene::Arena(Arena::from_map_and_key(ascii_map, map_key))
    }

    // Does current scene act on user input immediately (not governed by a game tick)?
    // NB: Move into Ui not Scene. Then move out of core engine entirely into Ui.
    pub fn tick_based(&self) -> crate::ui::TickStyle {
        match self {
            Self::Arena(scene) => scene.tick_based(),
            Self::Splash(scene) => scene.tick_based(),
            Self::CodingArena(scene) => scene.tick_based(),
        }
    }

    // Advance game state. Called when clock ticks or when user inputs.
    pub fn advance(&mut self, cmd: MoveCmd) -> SceneContinuation {
        // NB: Use the crate that makes it easy to inherit behaviour between enum variants.
        match self {
            Self::Arena(scene) => scene.advance(cmd),
            Self::Splash(scene) => scene.advance(cmd),
            Self::CodingArena(scene) => scene.advance(cmd),
        }
    }

    pub fn as_arena(&self) -> &Arena<MovementLogic> {
        match self {
            Self::Arena(arena) => &arena,
            Self::Splash(_splash) => panic!(),
            Self::CodingArena(scene) => &scene.init_arena,
        }
    }

    // Used for debugging. Ideally would avoid .as_play().
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_arena().as_ascii_rows()
    }
}
