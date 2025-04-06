use super::{SceneEnding, Continuation};

// Would be nice to remove if easy
use macroquad::prelude::*;

use crate::engine::input::Input;
use super::dialogue::*;

/// Splash message, any key to continue. E.g. New level, game over.
///
/// TODO: More general name for splash including title-only and dialogue?
#[derive(Clone, Debug)]
pub struct Splash {
    /// Next stage to go to after continue.

    // Text for current interstitial screen. Only in Splash.
    pub splash_text: String,
    pub dialogue: Dialogue, // If this works, will replace splash_text
}

impl Splash
{
    pub fn from_string(txt: String) -> Splash {
        Splash {
            splash_text: txt,
            dialogue: Dialogue { entries: vec![]},
        }
    }

    pub fn from_dialogue(entries: Vec<&str>) -> Splash {
        Splash {
            splash_text: "".to_string(),
            dialogue: Dialogue { entries: entries.iter().map(|x| DialogueLine {tex_path: "".to_string(), text: x.to_string()} ).collect() },
        }
    }

    pub fn advance(&mut self, input: &mut Input) -> SceneEnding {
        let key = input.consume_keypresses();

        // Reset "most recent tick" when leaving menu.
        // FIXME: Avoid needing input as a parameter, move time update to input code.
        input.last_real_update = get_time();

        match key {
            Some(_) => SceneEnding::Break(Continuation::SplashContinue),
            None => SceneEnding::Continue(()),
        }
    }
}
