use std::collections::HashMap;
use std::ops::ControlFlow;

use super::*;
use crate::engine::input::Input;
use crate::engine::obj::FreeObj;

// TODO: Move into game-specific info if possible?
// TODO: Rename PaneConclusion??
#[allow(dead_code)]
pub enum PaneEnding {
    SplashNext,
    PlayWin,
    PlayDie,
}

pub type PaneContinuation = ControlFlow<PaneEnding, ()>;

/// One unit of gameplay: one map layout, one splash screen, etc.
///
/// Would be nice to have base trait for pane types. Look for helper crate?
#[derive(Clone, Debug)]
pub enum Pane<ObjScriptProps: super::super::obj_scripting_properties::BaseObjScriptProps> {
    Play(Play<ObjScriptProps>),
    Splash(Splash),
}

impl<ObjScriptProps: super::super::obj_scripting_properties::BaseObjScriptProps> Pane<ObjScriptProps> {
    pub fn from_splash_string(txt: String) -> Self {
        Pane::Splash(Splash::from_string(txt))
    }

    pub fn from_splash_dialogue(entries: Vec<&str>) -> Self {
        Pane::Splash(Splash::from_dialogue(entries))
    }

    pub fn from_play_ascii_map<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<FreeObj<ObjScriptProps>>>,
    ) -> Self {
        Pane::Play(Play::from_ascii(ascii_map, map_key))
    }

    // Does current pane act on user input immediately (not governed by a game tick)?
    pub fn is_continuous(&self) -> bool {
        match self {
            Self::Splash(_) => true,
            Self::Play(_) => false,
        }
    }

    // Advance game state. Called when clock ticks or when user inputs.
    pub fn advance<Scripts: super::super::for_scripting::BaseScripts>(&mut self, input : &mut Input) -> PaneContinuation {
        match self {
            Self::Play(play) => play.advance::<Scripts>(input),
            Self::Splash(play) => play.advance(input),
        }
    }

    #[cfg(test)]
    pub fn as_play(&self) -> &Play<ObjScriptProps> {
        match self {
            Self::Play(play) => &play,
            Self::Splash(_splash) => panic!(),
        }
    }

    // Used for debugging. Ideally would avoid .as_play().
    #[cfg(test)]
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_play().field.as_ascii_rows()
    }
}
