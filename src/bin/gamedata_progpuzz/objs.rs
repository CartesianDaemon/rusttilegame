use macroquad::prelude::*;

use crate::engine::for_gamedata::*;
// TODO: Need to reference this game's version of scripts

pub fn new_hero_crab() -> FreeObj<super::obj_scripting_properties::DefaultObjScriptProps> {
    FreeObj {
        logical_props: LogicalProps::<obj_scripting_properties::DefaultObjScriptProps> {
            name:"Hero".to_string(),
            pass: Pass::Mov,
            ai: AI::Hero,
            .. LogicalProps::<obj_scripting_properties::DefaultObjScriptProps>::defaults()
        },
        visual_props: VisualProps::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_floor() -> FreeObj<super::obj_scripting_properties::DefaultObjScriptProps> {
    FreeObj {
        logical_props: LogicalProps::<obj_scripting_properties::DefaultObjScriptProps> {
            name: "Floor".to_string(),
            .. LogicalProps::<obj_scripting_properties::DefaultObjScriptProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> FreeObj<super::obj_scripting_properties::DefaultObjScriptProps> {
    FreeObj {
        logical_props: LogicalProps::<obj_scripting_properties::DefaultObjScriptProps> {
            name: "Wall".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<obj_scripting_properties::DefaultObjScriptProps>::defaults()
        },
        visual_props: VisualProps::new_col(DARKGRAY)
    }
}
