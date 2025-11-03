use macroquad::prelude::*;

use crate::engine::for_gamedata::*;
// TODO: Need to reference this game's version of scripts

pub fn new_hero_crab() -> FreeObj {
    FreeObj {
        logical_props: LogicalProps {
            name:"Hero".to_string(),
            pass: Pass::Mov,
            ai: AI::Hero,
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_floor() -> FreeObj {
    FreeObj {
        logical_props: LogicalProps {
            name: "Floor".to_string(),
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> FreeObj {
    FreeObj {
        logical_props: LogicalProps {
            name: "Wall".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps::new_col(DARKGRAY)
    }
}
