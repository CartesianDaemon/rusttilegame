/// Code for loading or instatiating each level.
///
/// Towards a generic, although currently game engine hardcodes BiobotLevs
/// and testing will hardcode a test set of levels.
///
/// Rename to "levelset" or similar?

use macroquad::prelude::*;

use crate::*;

use map_coords::CoordDelta;
// Need many of the specific params in ent.
// Some of those may move to his file.
use obj::*;
use play::Play;

/// Opaque base type for LevelSetDerived types
///
/// A separate type because trait object types can't be copy but the derived types
/// should be. Is that necessary?
///
/// Can I reduce the boilerplate in needing to trivially derive from both LevBase
/// and LevDerived?
///
/// Can remove LevStageDerived at all now LevStageBase uses DynClone as well as Downcast?
///
/// Kind of wants to be sized so it can easily be boxed and cloned etc. There's a crate for
/// that, is it worth trying?
///
/// Any benefit for adding a type or struct for Box<dyn LevelstageBase> as that's what we
/// pass around?
pub trait LevstageBase : downcast_rs::Downcast + dyn_clone::DynClone + std::fmt::Debug {
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
    ///
    /// Would be any easier to clone box?
    fn load_lev_stage(&self, lev_stage_box : &Box<dyn LevstageBase>) -> Play {
        if let Some(lev_stage) = lev_stage_box.downcast_ref::<Self::Levstage>() {
            self._load_lev_stage(*lev_stage)
        } else {
            panic!("Lev stage box -> lev stage cast failure");
        }
    }
}

// SPECIFIC ENT TYPES
// public only for helper use in test.rs
// maybe move to biobot.rs?

pub fn new_hero_crab() -> Obj {
    Obj {
        char: '*',
        name:"Hero".to_string(),
        pass: Pass::Mov,
        ai: AI::Hero,
        ..Obj::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_snake(dir: CoordDelta) -> Obj {
    Obj {
        char: 'f',
        name: "Fish".to_string(),
        pass: Pass::Mov,
        ai: AI::Bounce,
        dir: dir,
        effect: Effect::Kill,
        ..Obj::new_text_fill("FISH".to_string(), Some(DARKBLUE), Some(WHITE))
    }
}

pub fn new_floor() -> Obj {
    Obj {
        char: '.',
        name: "Floor".to_string(),
        ..Obj::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> Obj {
    Obj {
        char: 'W',
        name: "Wall".to_string(),
        pass: Pass::Solid,
        ..Obj::new_col(DARKGRAY)
    }
}

pub fn new_door_open() -> Obj {
    Obj {
        char: '_',
        name: "OpenDoor".to_string(),
        ..Obj::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> Obj {
    Obj {
        char: '#',
        name: "ClosedDoor".to_string(),
        pass: Pass::Solid,
        ..Obj::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> Obj {
    Obj {
        char: '!',
        name: "Goal".to_string(),
        effect: Effect::Win,

        border: Some(LIGHTGRAY),
        ..Obj::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
    }
}
