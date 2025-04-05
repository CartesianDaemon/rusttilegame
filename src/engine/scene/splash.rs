use super::Continuation;

// Would be nice to remove if easy
use macroquad::prelude::*;

use crate::engine::input::Input;
use super::dialogue::Dialogue;

/// Splash message, any key to continue. E.g. New level, game over.
#[derive(Clone, Debug)]
pub struct Splash {
    /// Next stage to go to after continue.

    // Text for current interstitial screen. Only in Splash.
    pub splash_text: String,
    pub dialogue: Dialogue, // If this works, will replace splash_text
}

impl Splash
{
    pub fn advance(&mut self, input: &mut Input) -> Option<Continuation> {
        let key = input.consume_keypresses();

        // Reset "most recent tick" when leaving menu.
        // FIXME: Avoid needing input as a parameter, move time update to input code.
        input.last_real_update = get_time();

        match key {
            Some(_) => Some(Continuation::SplashContinue),
            None => None,
        }
    }
}
