// Would be nice to remove if easy
use macroquad::prelude::*;

mod play;
mod splash;

mod dialogue;
pub use play::Play;
pub use splash::Splash;

use dialogue::{DialogueLine, Dialogue};

use std::collections::HashMap;

use super::input::Input;
use super::field::Field;
use super::obj::Obj;

pub enum Continuation {
    SplashContinue,
    PlayWin,
    PlayDie,
}

// TODO: Might be nice to make common base type trait for Play and Splash.

/// State of current scene: current level, map, etc.
///
/// Public fields should only be needed by Render or produced by load, not
/// used elsewhere.
///
/// Stores id of next stage through opaque LevelNumBase trait object. It was a pain to
/// get the trait object to work. Also consider using a fixed-size type for LevelNumBase.
/// Also considered making Play templated on Game at compile time.
///
/// Eventually we'll probably need to store the current Levstage.
#[derive(Clone, Debug)]
pub enum Scene {
    Play(Play),
    Splash(Splash),
}

impl Scene {
    pub fn make_splash(txt: String) -> Scene {
        Scene::Splash( Splash {
            splash_text: txt,
            dialogue: Dialogue { entries: vec![]},
        })
    }

    pub fn make_dialogue(entries: Vec<&str>) -> Scene {
        Scene::Splash( Splash {
            splash_text: "".to_string(),
            dialogue: Dialogue { entries: entries.iter().map(|x| DialogueLine {tex_path: "".to_string(), text: x.to_string()} ).collect() },
        })
    }

    // TODO: Move to Play
    // TODO: Do we need a function or would having levset_biobots use Play {...} be better?
    // TODO: Use lifetime or Rc on map_key instead of clone()?
    pub fn play_from_ascii<const HEIGHT: usize>(
        ascii_map: &[&str; HEIGHT],
        map_key: HashMap<char, Vec<Obj>>,
    ) -> Scene {
        // TODO: Get size from strings. Assert equal to default 16 in meantime.
        let mut play = Play {
            field: Field {
                map_key: map_key.clone(),
                ..Field::empty(ascii_map[0].len() as u16, HEIGHT as u16)
            },
        };

        for (y, line) in ascii_map.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                for ent in map_key.get(&ch).unwrap() {
                    play.spawn_at(x as i16, y as i16, ent.clone());
                }
            }
        }

        Scene::Play(play)
    }

    // Does current mode need UI to wait for tick before updating state?
    // Yes during play of level, no in splash screens.
    pub fn continuous(&self) -> bool {
        match self {
            Self::Splash(_) => true,
            Self::Play(_) => false,
        }
    }

    // Advance game state according to current state
    pub fn advance(&mut self, input : &mut Input) -> Option<Continuation> {
        match self {
            Self::Play(play) => play.advance(input.consume_keypresses()),
            Self::Splash(play) => play.advance(input),
        }
    }

    #[allow(dead_code)]
    pub fn as_play(&self) -> &Play {
        match self {
            Self::Play(play) => &play,
            Self::Splash(_splash) => panic!(),
        }
    }

    pub fn to_play_or_placeholder(&self) -> Play {
        match self {
            Self::Play(play) => play.clone(),
            Self::Splash(_splash) => Play {
                field: Field::empty(16, 16),
            },
        }
    }

    #[allow(dead_code)]
    pub fn as_ascii_cols(&self)-> Vec<String>  {
        self.as_play().field.as_ascii_cols()
    }

    #[allow(dead_code)]
    pub fn as_ascii_rows(&self)-> Vec<String>  {
        self.as_play().field.as_ascii_rows()
    }
}
