use std::collections::HashMap;
use std::ops::ControlFlow;

pub use crate::pane_arena::Arena;
pub use crate::pane_code::*;
pub use crate::pane_splash::*;
pub use crate::pane_split::*;

use crate::input::Input;
use crate::obj::FreeObj;

// TODO: Move into game-specific info if possible?
#[derive(Debug, PartialEq)]
pub enum PaneConclusion {
    SplashContinue,
    ArenaWin,
    ArenaDie,
}

// After each tick, either Continue, or restart/start another level based on Conclusion.
pub type PaneContinuation = ControlFlow<PaneConclusion, ()>;

// NB Breadcrumb: Need different name for Scene ("level part") than Pane ("screen part").
pub trait BasePane {
    fn need_sync_to_ticks(&self) -> bool;
    fn advance(&mut self, input : &mut Input) -> PaneContinuation;
}

/// One unit of gameplay: one map layout, one splash screen, etc.
///
/// Breadcrumb: Implement PaneBase using spire_enum or similar crate?
#[derive(Clone, Debug)]
pub enum Pane<MovementLogic: super::for_gamedata::BaseMovementLogic> {
    Arena(Arena<MovementLogic>),
    Splash(Splash),
    Split(Split<MovementLogic>),
    // Could be defined but not used separately:
    //  Code(Code)
}

impl<MovementLogic: super::for_gamedata::BaseMovementLogic> Pane<MovementLogic> {
    pub fn from_splash_string(txt: String) -> Self {
        Pane::Splash(Splash::from_string(txt))
    }

    pub fn from_splash_dialogue(entries: Vec<&str>) -> Self {
        Pane::Splash(Splash::from_dialogue(entries))
    }

    pub fn from_play_ascii_map<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<MovementLogic::CustomProps>>>,
    ) -> Self {
        Pane::Arena(Arena::from_ascii(ascii_map, map_key))
    }

    // Does current pane act on user input immediately (not governed by a game tick)?
    pub fn need_sync_to_ticks(&self) -> bool {
        match self {
            Self::Arena(pane) => pane.need_sync_to_ticks(),
            Self::Splash(pane) => pane.need_sync_to_ticks(),
            Self::Split(pane) => pane.need_sync_to_ticks(),
        }
    }

    // Advance game state. Called when clock ticks or when user inputs.
    pub fn advance(&mut self, input : &mut Input) -> PaneContinuation {
        // TODO: Was there a pattern to get a variable of trait type here and avoid repition?
        match self {
            Self::Arena(pane) => pane.advance(input),
            Self::Splash(pane) => pane.advance(input),
            Self::Split(pane) => pane.advance(input),
        }
    }

    pub fn as_arena(&self) -> &Arena<MovementLogic> {
        match self {
            Self::Arena(arena) => &arena,
            Self::Splash(_splash) => panic!(),
            Self::Split(pane) => &pane.arena,
        }
    }

    // Used for debugging. Ideally would avoid .as_play().
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_arena().map.as_ascii_rows()
    }
}
