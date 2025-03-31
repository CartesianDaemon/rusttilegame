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
use super::play::Scene;

/// Opaque base type for LevelNum types.
///
/// See the description on that trait. This might not be helping any more.
///
/// TODO: Also is there a more idiomatic name for a Stage/"Level" representing one screen?
///       Would it make more sense to define a level, and have that generate different screens
///       within?
///
/// A separate type because trait object types can't be copy but the derived types
/// should be. Is that necessary?
///
/// Older comments:
/// - Can remove LevStageDerived at all now LevStageBase uses DynClone as well as Downcast?
/// - Kind of wants to be sized so it can easily be boxed and cloned etc. There's a crate for
///   that, is it worth trying?
/// - Any benefit for adding a type or struct for Box<dyn GametageBase> as that's what we
///   pass around?
pub trait SceneIdBase : downcast_rs::Downcast + dyn_clone::DynClone + std::fmt::Debug {
}
downcast_rs::impl_downcast!(SceneIdBase);

dyn_clone::clone_trait_object!(SceneIdBase);

/// A trait describing classes which identify a level in a game (e.g. Level 1, etc)
///
/// Despite the name most games use it to identify part of a level, e.g. a level
/// splash screen, the level gameplay, etc. It is typically a rust enum.
///
/// It is derived from LevelNumBase so that the game engine can store level identifiers
/// through a dyn-trait pointer with the base trait type. And downcast them back to the
/// game-specific type to pass them back to game-specific code.
///
/// This seemed like a good idea to allow games to be dynamically loaded, but it might
/// be unnecessary and maybe it should be removed in favour of templating the Game
/// class on a game-specific Levset at compile time. (Or separating the game-specific
/// parts which need to be compiled, from those that could be dynamically loaded as data)
#[allow(dead_code)]
pub trait SceneId : SceneIdBase + Copy + Clone {
}

/// A trait describing classes which define the levels for a game.
///
/// This could be a simple array of levels, but it doesn't assume that so that
/// games could generate levels dynamically or load them from files.
///
/// The simplest implementations of Game hardcode the set of levels, e.g. the
/// the example Biobot game implements the trait on the BiobotGame class which
/// doesn't have any member variables at all.
///
/// However it is possible to have implementations that do have state, e.g. a
/// random seed used to generate a set of levels, or a filename that the levels
/// are loaded from.
///
/// It might simplify the implementation of the game engine slightly to specify the
/// size of objects implementing the Game trait so the game engine can store them
/// without dynamic allocation, but that probably doesn't gain much efficiency.
pub trait Game {
    type Levstage : SceneId;

    /// Level stage to begin game with
    fn initial_lev_stage(&self) -> Self::Levstage;

    /// Load or construct a Play instance for the specified level stage.
    fn load_lev_stage_impl(&self, lev_stage : Self::Levstage) -> Scene;

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
    fn load_lev_stage(&self, lev_stage_box : &Box<dyn SceneIdBase>) -> Scene {
        if let Some(lev_stage) = lev_stage_box.downcast_ref::<Self::Levstage>() {
            self.load_lev_stage_impl(*lev_stage)
        } else {
            panic!("Lev stage box -> lev stage cast failure");
        }
    }
}

// SPECIFIC OBJ TYPES
// Should be converted into helper functions in tests or in an example game. And game-specific
// functions moved to biobot/levels.rs.

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
