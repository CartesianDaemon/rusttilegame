use super::{Play};
use super::{PaneContinuation, PaneConclusion};
use crate::engine::input::Input;

#[derive(Clone, Debug)]
pub struct Code {
    // Palette of available instructions, array of assembled instructions, etc.
}

#[derive(Clone, Debug)]
pub struct Split<MovementLogic : super::super::for_gamedata::BaseMovementLogic> {
    pub play: Play<MovementLogic>,
    pub code: Code,
}

impl<MovementLogic : super::super::for_gamedata::BaseMovementLogic> Split<MovementLogic>
{
//    pub fn from_string(txt: String) -> Splash {
//        Splash {
//            splash_text: txt,
//            dialogue: Dialogue { entries: vec![]},
//        }
//    }

//    pub fn from_dialogue(entries: Vec<&str>) -> Splash {
//        Splash {
//            splash_text: "".to_string(),
//            dialogue: Dialogue { entries: entries.iter().map(|x| DialogueLine {tex_path: "".to_string(), text: x.to_string()} ).collect() },
//        }
//    }

    pub fn advance(&mut self, _input: &mut Input) -> PaneContinuation {
        // TODO

        return PaneContinuation::Continue(());
    }
}
