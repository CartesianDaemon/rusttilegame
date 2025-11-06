use macroquad::prelude::*;

use crate::engine::for_gamedata::*;
// TODO: Need to reference this game's version of scripts

type SimpleCustomProps = super::super::simple_custom_props::SimpleCustomProps;

pub fn new_hero_crab() -> FreeObj<super::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name:"Hero".to_string(),
            pass: Pass::Mov,
            ai: AI::Hero,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_floor() -> FreeObj<super::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Floor".to_string(),
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> FreeObj<super::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Wall".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col(DARKGRAY)
    }
}
