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
use super::play::{Scene, Continuation};

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
pub trait GameTrait {
    fn new_game() -> Self;

    /// Load or construct a Play instance for the specified level stage.
    fn load_lev_stage_impl(&mut self, continuation: Continuation) -> Scene;

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
    fn load_lev_stage(&mut self, continuation: Continuation) -> Scene {
        self.load_lev_stage_impl(continuation)
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
