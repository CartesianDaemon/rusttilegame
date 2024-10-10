use crate::types::Delta;

use macroquad::prelude::*;

// "Entity": Anything tile-sized and drawable including floor, wall, object, being.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Ent {
    // Cache of coords ent is at on map. These are useful for movement logic, but probably
    // aren't required. FIXME: Could be a Pos instead of separate coords.
    pub x: i16,
    pub y: i16,
    pub h: u16,

    /// Internal name for debugging
    pub name: String,

    // Visual display properties.
    // Only used by Render. Worth moving into a separate struct shared between Map and Render?
    // If we replace Texture with path and initialise texture only in Render, we should break
    // dependency on macroquad runtime code.
    pub border: Option<Color>,
    pub fill: Option<Color>,
    pub tex_path: Option<String>,
    pub text: Option<String>,
    pub text_col: Option<Color>,

    // Ent properties and behaviour, used by Game logic.

    // Solidity, e.g. wall, floor
    pub pass: Pass,

    // Movement control logic for enemies
    pub ai: AI,

    // Internal status for specific ent types.
    pub dir: Delta,

    // Effect of intersecting hero
    pub effect: Effect,
}

impl Ent {
    // An unitialised ent
    pub fn invalid() -> Ent {
        Ent {
            x: -1, // For now "-1" flags "this element is a placeholder in height vector"
            y: -1,
            h: 0,

            name: "????".to_string(),

            border: None,
            fill: None,
            tex_path: None,
            text: None,
            text_col: None,

            pass: Pass::Empty,
            ai: AI::Stay, // STUB: Could use this as a better placeholder flag
            effect: Effect::Nothing,

            dir: (0, 0),
        }
    }

    // An ent which is ignored when it exists in the map.
    pub fn placeholder() -> Ent {
        Ent::invalid()
    }

    // Default values for fields not used in a particular ent type.
    #[allow(dead_code)]
    pub fn empty() -> Ent {
        Ent {
            ..Ent::invalid()
        }
    }

    pub fn is_placeholder(&self) -> bool {
        self.x == -1
    }

    #[allow(dead_code)]
    pub fn new_tex(tex_path: String) -> Ent {
        Ent {
            h: 0, // Will be overridden
            tex_path: Some(tex_path),
            ..Ent::invalid()
        }
    }

    pub fn new_tex_col(tex_path: String, fill: Color) -> Ent {
        Ent {
            tex_path: Some(tex_path),
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    pub fn new_col(fill: Color) -> Ent {
        Ent {
            fill: Some(fill),
            ..Ent::invalid()
        }
    }

    pub fn new_col_outline(fill: Color, outline: Color) -> Ent {
        Ent {
            fill: Some(fill),
            border: Some(outline),
            ..Ent::invalid()
        }
    }

    pub fn new_text_fill(text: String, fill: Option<Color>, text_col: Option<Color>) -> Ent {
        Ent {
            text: Some(text),
            fill,
            text_col,
            ..Ent::invalid()
        }
    }

    // FUNCTIONS REFERRING TO SPECIFIC PROPERTIES
    // STUB: Could be combined if properties are made more generic.

    pub fn is_hero(self: &Ent) -> bool {
        self.ai == AI::Hero
    }

    // Indicate Ents which can move in their own logic, and need to be added to roster.
    pub fn is_roster(self: &Ent) -> bool {
        self.ai != AI::Hero && self.ai != AI::Stay
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
