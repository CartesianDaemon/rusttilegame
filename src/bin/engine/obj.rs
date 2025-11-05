use super::map_coords::CoordDelta;
use super::obj_scripting_properties;
use super::obj_scripting_properties::{BaseObjScriptProps, DefaultObjScriptProps};

use macroquad::prelude::*;

/// An Obj is anything tile-sized and drawable: floor, wall, object, being.
/// Representing an object not placed in the map. May not be used.
#[derive(Clone, Debug)]
pub struct FreeObj {
    pub logical_props: LogicalProps::<DefaultObjScriptProps>,
    pub visual_props: VisualProps,
}

/// Somewhat fuzzy match used for determining ascii representation.
/// Ideally would have a different name not PartialEq.
impl PartialEq for FreeObj {
    fn eq(&self, other:&Self) -> bool {
        self.logical_props == other.logical_props
    }
}

/// Logical properties of object, used for game logic and scripting.
/// Some of this could be moved into Gamedata? With base trait for required props?
#[derive(Clone, Debug, PartialEq)]
pub struct LogicalProps<ObjScriptProps: BaseObjScriptProps> {
    /// String representation of object, used internally for debug fmt etc.
    pub name: String,

    // Solidity, e.g. wall, floor
    pub pass: obj_scripting_properties::Pass,

    // Movement control logic for enemies
    pub ai: ObjScriptProps::AI,

    // Internal status for ents which have a current movement direction.
    // Also used for display
    pub dir: CoordDelta,

    // Effect of intersecting hero
    pub effect: obj_scripting_properties::Effect,
}

impl<ObjScriptProps: BaseObjScriptProps> LogicalProps<ObjScriptProps> {
    pub fn defaults() -> Self {
        Self {
            name: "????".to_string(),

            pass: obj_scripting_properties::Pass::Empty,
            ai: ObjScriptProps::AI::default(),
            effect: obj_scripting_properties::Effect::Nothing,

            dir: CoordDelta::from_xy(0, 0),
        }
    }

    // FUNCTIONS REFERRING TO SPECIFIC PROPERTIES
    // NB: Could be combined if properties are made more generic.

    // Todo: Replace with more meaningful "is_hero" fn in scripts. Or obj_properties??
    pub fn is_hero(ai: obj_scripting_properties::AI) -> bool {
        ai == obj_scripting_properties::AI::Hero
    }

    // Indicate Obj which can move in their own logic, and need to be added to roster.
    pub fn is_mob(ai: obj_scripting_properties::AI) -> bool {
        Self::is_any_mov(ai) && ! Self::is_hero(ai)
    }

    // Mob or Hero
    pub fn is_any_mov(ai: obj_scripting_properties::AI) -> bool {
        ai != obj_scripting_properties::AI::Stay
    }
}

/// Visual display properties. Only used by Render.
#[derive(Clone, Debug)]
pub struct VisualProps {
    pub border: Option<Color>,
    pub fill: Option<Color>,

    // For now, tex is animated all the time including stationary.
    // TODO: Consider AnimState which specifies which ones should be.
    pub tex_paths: Vec<String>,
    pub tex_scale: f32,

    pub text: Option<String>,
    pub text_col: Option<Color>,

    // logical_props::dir also used for display
}

impl VisualProps {
    pub fn defaults() -> Self {
        Self {
            border: None,
            fill: None,
            tex_paths: vec![],
            tex_scale: 1.0,
            text: None,
            text_col: None,
        }
    }

    pub fn assets_path() -> String {
        "imgs/".to_string()
    }

    #[allow(dead_code)]
    pub fn new_tex(tex_path: &str) -> Self {
        Self {
            tex_paths: vec![Self::assets_path() + tex_path],
            ..Self::defaults()
        }
    }

    pub fn new_tex_col(tex_path: &str, fill: Color) -> Self {
        Self {
            tex_paths: vec![Self::assets_path() + tex_path],
            fill: Some(fill),
            ..Self::defaults()
        }
    }

    /// TODO: Need to add "rotate" option for directional movs.
    /// TODO: Fix path in wasm. No prefix?
    /// TODO: Bigger fish?
    pub fn new_tex_anim(tex_paths: Vec<&str>) -> Self {
        Self {
            // TODO: Consider using a list comprehension crate
            // TODO: Consider implementing my abbreviated map chain crate.
            //       Note whether that could usefully do .iter() and .collect()?
            // TODO: Consider whether simpler for caller to offer wildcard like "FishB*.png"
            // TODO: Consider where to specify path to imgs? Here? As part of levset?
            tex_paths: tex_paths.iter().map(|x| Self::assets_path() + x).collect(),
            ..Self::defaults()
        }
    }

    pub fn new_col(fill: Color) -> Self {
        Self {
            fill: Some(fill),
            ..Self::defaults()
        }
    }

    pub fn new_col_outline(fill: Color, outline: Color) -> Self {
        Self {
            fill: Some(fill),
            border: Some(outline),
            ..Self::defaults()
        }
    }

    pub fn new_text_fill(text: String, fill: Option<Color>, text_col: Option<Color>) -> Self {
        Self {
            text: Some(text),
            fill,
            text_col,
            ..Self::defaults()
        }
    }
}
