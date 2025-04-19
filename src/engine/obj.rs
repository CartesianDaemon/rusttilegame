use super::map_coords::CoordDelta;
use super::field::{RosterHandle};
use super::scripting::MapCoord;

use crate::scripts::*;

use macroquad::prelude::*;

/// Anything tile-sized and drawable including floor, wall, object, being.
/// TODO: Could have a separate "Object in map" or "active object" representation in map?
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Obj {
    // RichMapHandle is necessary to update object. Needs api?
    // Invalid is only used before placing in map?
    // TODO: Consider moving curr_handle, curr_pos, prev_pos into a sister struct in Field,
    //       only used by objs which are actually in the map.
    pub curr_roster_handle: RosterHandle,

    // curr_pos and prev_pos should only be informative. Kept up to date by field. Needs api?
    // Invalid is only used before placing in map? (And for placeholders for heights?)
    pub curr_pos: MapCoord,
    pub prev_pos: MapCoord,

    /// String representation of object, used internally for debug fmt etc.
    pub name: String,

    // Visual display properties.
    // Only used by Render. Worth moving into a separate struct shared between Map and Render?
    // If we replace Texture with path and initialise texture only in Render, we should break
    // dependency on macroquad runtime code.
    pub border: Option<Color>,
    pub fill: Option<Color>,

    // For now, tex is animated all the time including stationary.
    // TODO: Consider AnimState which specifies which ones should be.
    pub tex_paths: Vec<String>,
    pub tex_scale: f32,

    pub text: Option<String>,
    pub text_col: Option<Color>,

    // Ent properties and behaviour, used by Game logic.

    // Solidity, e.g. wall, floor
    pub pass: Pass,

    // Movement control logic for enemies
    pub ai: AI,

    // Internal status for ents which have a current movement direction.
    pub dir: CoordDelta,

    // Effect of intersecting hero
    pub effect: Effect,
}

impl Obj {
    // An unitialised ent
    pub fn invalid() -> Obj {
        Obj {
            curr_roster_handle: RosterHandle::invalid(),
            curr_pos: MapCoord::invalid(),
            prev_pos: MapCoord::from_xy(-1, -1),

            name: "????".to_string(),

            border: None,
            fill: None,
            tex_paths: vec![],
            tex_scale: 1.0,
            text: None,
            text_col: None,

            pass: Pass::Empty,
            ai: AI::Stay, // STUB: Could use this as a better placeholder flag
            effect: Effect::Nothing,

            dir: CoordDelta::from_xy(0, 0),
        }
    }

    // An ent which is ignored when it exists in the map.
    pub fn placeholder() -> Obj {
        Obj::invalid()
    }

    // Default values for fields not used in a particular ent type.
    #[allow(dead_code)]
    pub fn empty() -> Obj {
        Obj {
            ..Obj::invalid()
        }
    }

    pub fn is_placeholder(&self) -> bool {
        self.curr_pos == MapCoord::invalid()
    }

    pub fn assets_path() -> String {
        "imgs/".to_string()
    }

    #[allow(dead_code)]
    pub fn new_tex(tex_path: &str) -> Obj {
        Obj {
            tex_paths: vec![Self::assets_path() + tex_path],
            ..Obj::invalid()
        }
    }

    pub fn new_tex_col(tex_path: &str, fill: Color) -> Obj {
        Obj {
            tex_paths: vec![Self::assets_path() + tex_path],
            fill: Some(fill),
            ..Obj::invalid()
        }
    }

    /// TODO: Need to add "rotate" option for directional movs.
    /// TODO: Fix path in wasm. No prefix?
    /// TODO: Bigger fish?
    pub fn new_tex_anim(tex_paths: Vec<&str>) -> Obj {
        Obj {
            // TODO: Consider using a list comprehension crate
            // TODO: Consider implementing my abbreviated map chain crate.
            //       Note whether that could usefully do .iter() and .collect()?
            // TODO: Consider whether simpler for caller to offer wildcard like "FishB*.png"
            // TODO: Consider where to specify path to imgs? Here? As part of levset?
            tex_paths: tex_paths.iter().map(|x| Self::assets_path() + x).collect(),
            ..Obj::invalid()
        }
    }

    pub fn new_col(fill: Color) -> Obj {
        Obj {
            fill: Some(fill),
            ..Obj::invalid()
        }
    }

    pub fn new_col_outline(fill: Color, outline: Color) -> Obj {
        Obj {
            fill: Some(fill),
            border: Some(outline),
            ..Obj::invalid()
        }
    }

    pub fn new_text_fill(text: String, fill: Option<Color>, text_col: Option<Color>) -> Obj {
        Obj {
            text: Some(text),
            fill,
            text_col,
            ..Obj::invalid()
        }
    }

    // FUNCTIONS REFERRING TO SPECIFIC PROPERTIES
    // STUB: Could be combined if properties are made more generic.

    pub fn is_hero(self: &Obj) -> bool {
        self.ai == AI::Hero
    }

    // Indicate Obj which can move in their own logic, and need to be added to roster.
    pub fn is_mob(self: &Obj) -> bool {
        self.is_any_mov() && ! self.is_hero()
    }

    // Mob or Hero
    pub fn is_any_mov(self: &Obj) -> bool {
        self.ai != AI::Stay
    }

    fn comparable_fields(&self) -> (&String, &CoordDelta, &AI, &Pass) {
        (&self.name, &self.dir, &self.ai, &self.pass)
    }
}

/// Somewhat fuzzy match used for determining ascii representation.
/// Ideally would have a different name not PartialEq.
impl PartialEq for Obj {
    fn eq(&self, other:&Self) -> bool {
        self.comparable_fields() == other.comparable_fields()
    }
}
