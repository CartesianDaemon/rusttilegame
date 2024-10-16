use crate::map_coords::{CoordDelta, MapHandle};

use macroquad::prelude::*;

/// Anything tile-sized and drawable including floor, wall, object, being.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Obj {
    // Cache of coords ent is at on map. These are useful for movement logic, but probably
    // aren't required.
    pub cached_pos: MapHandle,
    // pub ros_idx: MapHandle,

    /// String representation of object, used internally for debug fmt etc.
    pub name: String,
    /// Single-character representation of object, used internally for ascii maps etc.
    pub char: char,

    // Visual display properties.
    // Only used by Render. Worth moving into a separate struct shared between Map and Render?
    // If we replace Texture with path and initialise texture only in Render, we should break
    // dependency on macroquad runtime code.
    pub border: Option<Color>,
    pub fill: Option<Color>,
    pub tex_paths: Vec<String>,
    pub tex_scale: f32,
    pub text: Option<String>,
    pub text_col: Option<Color>,

    /// Previous pos, expressed as handle (i.e coords and height)
    /// Height only relevant compared to prev_pos of other objs.
    pub prev_pos: MapHandle,

    // Ent properties and behaviour, used by Game logic.

    // Solidity, e.g. wall, floor
    pub pass: Pass,

    // Movement control logic for enemies
    pub ai: AI,

    // Internal status for specific ent types.
    pub dir: CoordDelta,

    // Effect of intersecting hero
    pub effect: Effect,
}

impl Obj {
    // An unitialised ent
    pub fn invalid() -> Obj {
        Obj {
            cached_pos: MapHandle::invalid(),

            name: "????".to_string(),
            char: '?',

            border: None,
            fill: None,
            tex_paths: vec![],
            tex_scale: 1.0,
            text: None,
            text_col: None,

            prev_pos: MapHandle::from_xyh(-1, -1, 0),

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
        self.cached_pos == MapHandle::invalid()
    }

    #[allow(dead_code)]
    pub fn new_tex(tex_path: String) -> Obj {
        Obj {
            tex_paths: vec![tex_path],
            ..Obj::invalid()
        }
    }

    pub fn new_tex_col(tex_path: String, fill: Color) -> Obj {
        Obj {
            tex_paths: vec![tex_path],
            fill: Some(fill),
            ..Obj::invalid()
        }
    }

    /// TODO: Need to add "rotate" option for directional movs.
    /// TODO: Fix path in wasm. No prefix?
    /// TODO: Bigger fish?
    pub fn new_tex_anim(tex_paths: Vec<&str>) -> Obj {
        let assets_path: &str = "imgs/";
        Obj {
            // TODO: Consider using a list comprehension crate
            // TODO: Consider implementing my abbreviated map chain crate.
            //       Note whether that could usefully do .iter() and .collect()?
            // TODO: Consider whether simpler for caller to offer wildcard like "FishB*.png"
            // TODO: Consider where to specify path to imgs? Here? As part of levset?
            tex_paths: tex_paths.iter().map(|x| assets_path.to_string() + x).collect(),
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
    pub fn is_other_mov(self: &Obj) -> bool {
        self.is_any_mov() && ! self.is_hero()
    }

    pub fn is_any_mov(self: &Obj) -> bool {
        self.ai != AI::Stay
    }
}

// STUB: Properties that can be applied to Ent.
// Makes more sense if there's a mod with Ent class and non-public classes only used inside it.
// Can these become a generic properties defined in load, rather than being specified by engine?

// Passable. Whether other movs can move through an ent or not.
#[derive(Clone, PartialEq, Debug)]
pub enum Pass {
    Empty, // No impediment to movement, e.g. floor.
    Solid, // Block movement, e.g. wall.
    Mov, // Something which can move itself, e.g. hero, enemy
    // INSERT: Obj, // Something which can be moved or maybe coexisted with, e.g. furniture
}

// Types of movement-control logic ents can use
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum AI {
    Stay, // No self movement. Not added to Roster's list of movs.
    Hero, // Controlled by keys. Assume only one hero, added to Roster's hero entry.
    // Everything else may spontaneously move or need to be enumerated, ie needs to be added to roster.
    Snake, // Move in direction, move orthogonally towards hero. Maybe: bounce off walls.
    Bounce, // Move in direction, reverse direction at walls.
}

// Effect when intersect with hero (as mov or stay)
#[derive(Clone, PartialEq, Debug)]
pub enum Effect {
    Nothing,
    Kill,
    Win,
    // STUB: Can add effects like when ent dies
    // STUB: Could convert Win, Kill, to Progress(Win),... with enum Progress {Win, Lose}
}
