use super::{PaneContinuation, PaneConclusion};
use super::BasePane;

// Would be nice to remove if easy
use macroquad::prelude::*;

use crate::input::Input;
use super::dialogue::*;

/// Splash message, any key to continue. E.g. New level, game over.
///
/// TODO: A name more like "announcement" or "interstitial" or..?
/// TODO: Merge rendering of single string with dialogue?
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
}

impl BasePane for Splash {
    fn advance(&mut self, input: &mut Input) -> PaneContinuation {
        let key = input.consume_cmd();

        // Reset "most recent tick" when leaving menu.
        // FIXME: Avoid needing input as a parameter, move time update to input code.
        input.last_real_update = get_time();

        match key {
            Some(_) => PaneContinuation::Break(PaneConclusion::SplashNext),
            None => PaneContinuation::Continue(()),
        }
    }

    fn need_sync_to_ticks(&self) -> bool {
        false
    }
}
