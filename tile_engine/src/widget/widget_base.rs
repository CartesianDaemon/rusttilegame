use std::collections::HashMap;
use std::ops::ControlFlow;

pub use super::arena::Arena;
pub use super::coding::*;
pub use super::splash::*;
pub use super::coding_arena::*;

use crate::map_coords::MoveCmd;
use crate::obj::FreeObj;
use crate::for_gamedata;

// TODO: Move into game-specific info if possible?
#[derive(Debug, PartialEq)]
pub enum WidgetConclusion {
    SplashContinue,
    ArenaWin,
    ArenaDie,
}

// After each tick, either Continue, or restart/start another level based on Conclusion.
pub type PaneContinuation = ControlFlow<WidgetConclusion, ()>;

// NB Breadcrumb: Need different name for Scene ("level part") than Pane ("screen part").
pub trait BaseWidget {
    fn tick_based(&self) -> bool;
    fn advance(&mut self, cmd: Option<MoveCmd>) -> PaneContinuation;
}

/// One unit of gameplay: one map layout, one splash screen, etc.
///
/// NB Breadcrumb: Refactor:
/// * Game state (dialogue, map, etc) stored in Pane is renamed Widget and
///   become (optional) parts of Gamedata.
/// * Gamedata maybe renamed State.
/// * LevID becomes entirely internal to Gamestate. How does encapsulation
///   for reusing widgets work? Maybe GameLogic for Gamestate interprets
///   Conclusions returned by GameLogic(s) for Widgets?
/// * Maybe "Screen" is a collection of "Panes", panes roughly corresponding
///   to renderers associated with one or more widgets?
/// * Components like Widgets, Renderers, etc become more mix and match
///   with some widely used ones implemented by Engine and others as add-ons
///   or in individual games??
/// Breadcrumb: Implement PaneBase using spire_enum or similar crate?
#[derive(Clone, Debug)]
pub enum Widget<GameLogic: for_gamedata::BaseGameLogic> {
    Arena(Arena<GameLogic>),
    Splash(Splash),
    CodingArena(CodingArena<GameLogic>),
    // Could be defined but not used separately:
    //  Code(Code)
}

impl<GameLogic: for_gamedata::BaseGameLogic> Widget<GameLogic> {
    pub fn from_splash_string(txt: String) -> Self {
        Widget::Splash(Splash::from_string(txt))
    }

    pub fn from_splash_dialogue(entries: Vec<&str>) -> Self {
        Widget::Splash(Splash::from_dialogue(entries))
    }

    pub fn from_play_ascii_map<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<GameLogic::CustomProps>>>,
    ) -> Self {
        Widget::Arena(Arena::from_ascii(ascii_map, map_key))
    }

    // Does current pane act on user input immediately (not governed by a game tick)?
    pub fn tick_based(&self) -> bool {
        match self {
            Self::Arena(pane) => pane.tick_based(),
            Self::Splash(pane) => pane.tick_based(),
            Self::CodingArena(pane) => pane.tick_based(),
        }
    }

    // Advance game state. Called when clock ticks or when user inputs.
    pub fn advance(&mut self, cmd: Option<MoveCmd>) -> PaneContinuation {
        // TODO: Was there a pattern to get a variable of trait type here and avoid repition?
        match self {
            Self::Arena(pane) => pane.advance(cmd),
            Self::Splash(pane) => pane.advance(cmd),
            Self::CodingArena(pane) => pane.advance(cmd),
        }
    }

    pub fn as_arena(&self) -> &Arena<GameLogic> {
        match self {
            Self::Arena(arena) => &arena,
            Self::Splash(_splash) => panic!(),
            Self::CodingArena(pane) => &pane.arena,
        }
    }

    // Used for debugging. Ideally would avoid .as_play().
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_arena().as_ascii_rows()
    }
}
