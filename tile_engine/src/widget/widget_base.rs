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
pub enum WidgetConclusion {
    SplashContinue,
    Win,
    Die,
}

// After each tick, either Continue, or restart/start another level based on Conclusion.
pub type WidgetContinuation = ControlFlow<WidgetConclusion, ()>;

// NB Breadcrumb: Need different name for Scene ("level part") than Pane ("screen part").
pub trait BaseWidget {
    fn tick_based(&self) -> crate::ui::TickStyle;
    fn advance(&mut self, cmd: MoveCmd) -> WidgetContinuation;
}

/// One unit of gameplay: one map layout, one splash screen, etc.
///
/// Often but not always one UI is a way to draw one corresponding widget.
///
/// Could make game-specific state more modularly include which widgets it
/// wants to use.
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
        Widget::Arena(Arena::from_map_and_key(ascii_map, map_key))
    }

    // Does current pane act on user input immediately (not governed by a game tick)?
    // NB: Move into Ui not Widget. Then move out of core engine entirely into Ui.
    pub fn tick_based(&self) -> crate::ui::TickStyle {
        match self {
            Self::Arena(widget) => widget.tick_based(),
            Self::Splash(widget) => widget.tick_based(),
            Self::CodingArena(widget) => widget.tick_based(),
        }
    }

    // Advance game state. Called when clock ticks or when user inputs.
    pub fn advance(&mut self, cmd: MoveCmd) -> WidgetContinuation {
        // NB: Use the crate that makes it easy to inherit behaviour between enum variants.
        match self {
            Self::Arena(widget) => widget.advance(cmd),
            Self::Splash(widget) => widget.advance(cmd),
            Self::CodingArena(widget) => widget.advance(cmd),
        }
    }

    pub fn as_arena(&self) -> &Arena<GameLogic> {
        match self {
            Self::Arena(arena) => &arena,
            Self::Splash(_splash) => panic!(),
            Self::CodingArena(pane) => &pane.init_arena,
        }
    }

    // Used for debugging. Ideally would avoid .as_play().
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_arena().as_ascii_rows()
    }
}
