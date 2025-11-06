use macroquad::prelude::*;

use crate::engine::for_gamedata::*;
// TODO: Need to reference this game's version of scripts

pub fn new_hero_crab() -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name:"Hero".to_string(),
            pass: Pass::Mov,
            ai: AI::Hero,
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_floor() -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "Floor".to_string(),
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "Wall".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col(DARKGRAY)
    }
}
