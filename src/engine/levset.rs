/// Code for loading or instatiating each level.
///
/// Towards a generic, although currently game engine hardcodes BiobotLevs
/// and testing will hardcode a test set of levels.
///
/// Rename to "levelset" or similar?

use macroquad::prelude::*;

use super::map_coords::CoordDelta;
// Need many of the specific params in ent.
// Some of those may move to his file.
use super::obj::*;
use super::play::Play;

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
pub trait LevelNumBase : downcast_rs::Downcast + dyn_clone::DynClone + std::fmt::Debug {
}
downcast_rs::impl_downcast!(LevelNumBase);

dyn_clone::clone_trait_object!(LevelNumBase);

/// Identify a level within a level set.
///
/// Stored in the game engine as a box dyn trait object because each type of level
/// set can use a different type to identify levels. Typically an enum combining
/// level number with intro/play/outro, and a few special states like GameOver.
/// For level sets loaded dynamically from a file, will use a general type like
/// a string.
#[allow(dead_code)]
pub trait LevelNum : LevelNumBase + Copy + Clone {
}

/// A trait describing classes which define the levels for a game.
///
/// This could be a simple array of levels, but it doesn't assume that so that
/// games could generate levels dynamically or load them from files.
///
/// The simplest implementations of Levels hardcode the set of levels, e.g. the
/// the example Biobot game implements the trait on the BiobotLevels class which
/// doesn't have any member variables at all.
///
/// However it is possible to have implementations that do have state, e.g. a
/// random seed used to generate a set of levels, or a filename that the levels
/// are loaded from.
///
/// It might simplify the implementation of the game engine slightly to specify the
/// size of objects implementing the Levels trait so the game engine can store them
/// without dynamic allocation, but that probably doesn't gain much efficiency.
pub trait Levels {
    type Levstage : LevelNum;

    /// Level stage to begin game with
    fn initial_lev_stage(&self) -> Self::Levstage;

    /// Load or construct a Play instance for the specified level stage.
    fn load_lev_stage_impl(&self, lev_stage : Self::Levstage) -> Play;

    /// Load or construct a Play instance for the specified level stage.
    ///
    /// Default implementation downcasts a LevelNumBase ref to the actual Levstage
    /// type and delegates the actual work to _load_lev_stage.
    ///
    /// Must accept box to do the downcasting.
    ///
    /// Accepts ref to box. Why can't we borrow a box?
    ///
    /// Would be any easier to clone box?
    fn load_lev_stage(&self, lev_stage_box : &Box<dyn LevelNumBase>) -> Play {
        if let Some(lev_stage) = lev_stage_box.downcast_ref::<Self::Levstage>() {
            self.load_lev_stage_impl(*lev_stage)
        } else {
            panic!("Lev stage box -> lev stage cast failure");
        }
    }
}

// SPECIFIC OBJ TYPES
// public only for helper use in test.rs
// maybe move to biobot.rs?

pub fn new_hero_crab() -> Obj {
    Obj {
        name:"Hero".to_string(),
        pass: Pass::Mov,
        ai: AI::Hero,
        ..Obj::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_fish(dir: CoordDelta) -> Obj {
    Obj {
        name: "Fish".to_string(),
        pass: Pass::Mov,
        ai: AI::Bounce,
        dir: dir,
        effect: Effect::Kill,
        tex_scale: 1.7,
        ..Obj::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
    }
}

pub fn new_gawpie(dir: CoordDelta) -> Obj {
    Obj {
        name: "Gawpie".to_string(),
        pass: Pass::Mov,
        ai: AI::Drift,
        dir: dir,
        effect: Effect::Kill,
        tex_scale: 1.7,
        ..Obj::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
    }
}

pub fn new_floor() -> Obj {
    Obj {
        name: "Floor".to_string(),
        ..Obj::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> Obj {
    Obj {
        name: "Wall".to_string(),
        pass: Pass::Solid,
        ..Obj::new_col(DARKGRAY)
    }
}

pub fn new_door_open() -> Obj {
    Obj {
        name: "OpenDoor".to_string(),
        ..Obj::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> Obj {
    Obj {
        name: "ClosedDoor".to_string(),
        pass: Pass::Solid,
        ..Obj::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> Obj {
    Obj {
        name: "Goal".to_string(),
        effect: Effect::Win,

        border: Some(LIGHTGRAY),
        ..Obj::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
    }
}
