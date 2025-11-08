use super::map_coords::CoordDelta;
use super::super::simple_custom_props;
use super::for_gamedata::BaseCustomProps;

use macroquad::prelude::*;

/// An Obj is anything tile-sized and drawable: floor, wall, object, being.
/// Representing an object not placed in the map. May not be used.
#[derive(Clone, Debug)]
pub struct FreeObj<CustomProps: BaseCustomProps> {
    pub logical_props: LogicalProps::<CustomProps>,
    pub visual_props: VisualProps,
}

/// Somewhat fuzzy match used for determining ascii representation.
/// Ideally would have a different name not PartialEq.
impl<CustomProps: BaseCustomProps> PartialEq for FreeObj<CustomProps> {
    fn eq(&self, other:&Self) -> bool {
        self.logical_props == other.logical_props
    }
}

/// Logical properties of object, used for game logic and scripting.
/// Some of this could be moved into Gamedata? With base trait for required props?
#[derive(Clone, Debug, PartialEq)]
pub struct LogicalProps<CustomProps: BaseCustomProps> {
    /// String representation of object, used internally for debug fmt etc.
    pub name: String,

    // Internal status for ents which have a current movement direction.
    // Also used for display
    pub dir: CoordDelta,

    // Custom properties used by movement logic.
    pub custom_props: CustomProps,

    // Solidity, e.g. wall, floor
    // TODO: Eventually merge into custom_props.
    pub pass: simple_custom_props::Pass,

    // Effect of intersecting hero
    // TODO: Eventually merge into custom_props.
    pub effect: simple_custom_props::Effect,
}

impl<CustomProps: BaseCustomProps> LogicalProps<CustomProps> {
    pub fn defaults() -> Self {
        Self {
            name: "????".to_string(),

            dir: CoordDelta::from_xy(0, 0),

            custom_props: CustomProps::default(),

            pass: simple_custom_props::Pass::Empty,
            effect: simple_custom_props::Effect::Nothing,

        }
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
