use std::collections::HashMap;
use std::ops::ControlFlow;

use super::*;
use crate::engine::input::Input;
use crate::engine::field::Field;
use crate::engine::obj::Obj;

/// How a scene ended, used to tell which to go to next
pub enum Continuation {
    SplashContinue,
    PlayWin,
    PlayDie,
}

pub enum SceneEnding {
    NextScene(Continuation),
    ContinuePlaying
}

impl std::ops::FromResidual<Continuation> for SceneEnding {
    fn from_residual(continuation: Continuation) -> Self {
        SceneEnding::NextScene(continuation)
    }
}

impl std::ops::Try for SceneEnding {
    type Output = ();
    type Residual = Continuation;

    fn from_output(_: Self::Output) -> Self {
        SceneEnding::ContinuePlaying
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            SceneEnding::ContinuePlaying => ControlFlow::Continue(()),
            SceneEnding::NextScene(continuation) => ControlFlow::Break(continuation),
        }
    }
}

// TODO: Might be nice to make common base type trait for Play and Splash.
// TODO: Or even move Scene types into a helper directory somewhere between
//       game engine and individual game.

/// State of current scene: current level, map, etc.
///
/// Public fields should only be needed by Render or produced by load, not
/// used elsewhere.
///
/// TODO: Using Enum of alternate structs pattern, consider looking for helper crate?
#[derive(Clone, Debug)]
pub enum Scene {
    Play(Play),
    Splash(Splash),
}

impl Scene {
    pub fn from_splash_string(txt: String) -> Scene {
        Scene::Splash(Splash::from_string(txt))
    }

    pub fn from_splash_dialogue(entries: Vec<&str>) -> Scene {
        Scene::Splash(Splash::from_dialogue(entries))
    }

    pub fn from_play_ascii_map<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<Obj>>,
    ) -> Scene {
        Scene::Play(Play::from_ascii(ascii_map, map_key))
    }

    // Does current mode need UI to wait for tick before updating state?
    // Yes during play of level, no in splash screens.
    // TODO: Move into play and splash structs.
    pub fn continuous(&self) -> bool {
        match self {
            Self::Splash(_) => true,
            Self::Play(_) => false,
        }
    }

    // Advance game state according to current state
    // TODO: Consider implementing common interface from input to structs?
    pub fn advance(&mut self, input : &mut Input) -> SceneEnding {
        match self {
            Self::Play(play) => play.advance(input.consume_keypresses()),
            Self::Splash(play) => play.advance(input),
        }
    }

    // TODO: Consider trying to remove necessity?
    #[allow(dead_code)]
    pub fn as_play(&self) -> &Play {
        match self {
            Self::Play(play) => &play,
            Self::Splash(_splash) => panic!(),
        }
    }

    // TODO: Consider trying to remove necessity?
    pub fn to_play_or_placeholder(&self) -> Play {
        match self {
            Self::Play(play) => play.clone(),
            Self::Splash(_splash) => Play {
                field: Field::empty(16, 16),
            },
        }
    }

    // TODO: Move into play. Still needed?
    #[allow(dead_code)]
    pub fn as_ascii_cols(&self)-> Vec<String>  {
        self.as_play().field.as_ascii_cols()
    }

    // TODO: Move into play. Still needed?
    #[allow(dead_code)]
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_play().field.as_ascii_rows()
    }
}
