/// Code for loading or instatiating each level.
///
/// Towards a generic, although currently game engine hardcodes BiobotLevs
/// and testing will hardcode a test set of levels.

use macroquad::prelude::*;

use std::collections::HashMap;

use crate::*;

use types::Delta;
// Need many of the specific params in ent.
// Some of those may move to his file.
use ent::*;
use play::Play;
use play::Mode;
use util::*;

/// Opaque base type for LevelSetDerived types
///
/// A separate type because trait object types can't be copy but the derived types
/// should be. Is that necessary?
///
/// Can I reduce the boilerplate in needing to trivially derive from both LevBase
/// and LevDerived?
///
/// Can remove LevStageDerived at all now LevStageBase uses DynClone as well as Downcast?
pub trait LevstageBase : downcast_rs::Downcast + dyn_clone::DynClone {
}
downcast_rs::impl_downcast!(LevstageBase);

dyn_clone::clone_trait_object!(LevstageBase);

/// Identify a level within a level set.
///
/// Stored in the game engine as a box dyn trait object because each type of level
/// set can use a different type to identify levels. Typically an enum combining
/// level number with intro/play/outro, and a few special states like GameOver.
/// For level sets loaded dynamically from a file, will use a general type like
/// a string.
#[allow(dead_code)]
pub trait LevstageDerived : LevstageBase + Copy + Clone {
}

/// A set of levels constituting one game.
///
/// Built in level sets typically have no state. May implement a dynamic level
/// set which loads from a file, where an instance would contain the file name,
/// or contents from the file.
pub trait LevSet {
    type Levstage : LevstageDerived;

    /// Level stage to begin game with
    fn initial_lev_stage(&self) -> Self::Levstage;

    /// Load or construct a Play instance for the specified level stage.
    fn _load_lev_stage(&self, lev_stage : Self::Levstage) -> Play;

    /// Load or construct a Play instance for the specified level stage.
    ///
    /// Default implementation downcasts a LevstageBase ref to the actual Levstage
    /// type and delegates the actual work to _load_lev_stage.
    ///
    /// Must accept box to do the downcasting.
    ///
    /// Accepts ref to box. Why can't we borrow a box?
    fn load_lev_stage(&self, lev_stage_box : &Box<dyn LevstageBase>) -> Play {
        if let Some(lev_stage) = lev_stage_box.downcast_ref::<Self::Levstage>() {
            self._load_lev_stage(*lev_stage)
        } else {
            panic!("Lev stage box -> lev stage cast failure");
        }
    }
}

////////////////////
// LevSet helpers

// Replace with Play constructor directly, or if useful make this a Play fn not a Stage fn?
pub fn make_splash(txt: String, to_stage: biobot::BiobotStage) -> Play {
    Play {
        mode: Mode::Splash,
        splash_text: txt,
        to_stage: Box::new(to_stage),
        die_stage: Box::new(biobot::BiobotStage::NewGame), // Shouldn't be used?
        ..Play::new_null_level()
    }
}

// Used by tests
pub fn make_levplay(levno: u16, ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
    Play {
        mode : Mode::LevPlay,
        to_stage: Box::new(biobot::BiobotStage::LevOutro(levno)),
        die_stage: Box::new(biobot::BiobotStage::Retry(levno)),
        ..Play::from_ascii(&ascii_map, map_key)
    }
}

// SPECIFIC ENT TYPES
// public only for helper use in test.rs

pub fn new_hero_crab() -> Ent {
    Ent {
        pass: Pass::Mov,
        ai: AI::Hero,
        ..Ent::new_tex_col(load_texture_blocking_unwrap("imgs/ferris.png"), GOLD)
    }
}

pub fn new_snake(dir: Delta) -> Ent {
    Ent {
        pass: Pass::Mov,
        ai: AI::Bounce,
        dir: dir,
        effect: Effect::Kill,
        ..Ent::new_col(DARKGREEN)
    }
}

pub fn new_floor() -> Ent {
    Ent {
        ..Ent::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> Ent {
    Ent {
        pass: Pass::Solid,
        ..Ent::new_col(DARKGRAY)
    }
}

pub fn new_door_open() -> Ent {
    Ent {
        ..Ent::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> Ent {
    Ent {
        pass: Pass::Solid,
        ..Ent::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> Ent {
    Ent {
        effect: Effect::Win,
        ..Ent::new_col_outline(GOLD, LIGHTGRAY)
    }
}
