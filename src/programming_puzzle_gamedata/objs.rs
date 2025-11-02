use macroquad::prelude::*;

use crate::engine::customgame::*;
// TODO: Need to reference this game's version of scripts

pub fn new_hero_crab() -> ObjProperties {
    ObjProperties {
        name:"Hero".to_string(),
        pass: Pass::Mov,
        ai: AI::Hero,
        ..ObjProperties::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_floor() -> ObjProperties {
    ObjProperties {
        name: "Floor".to_string(),
        ..ObjProperties::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> ObjProperties {
    ObjProperties {
        name: "Wall".to_string(),
        pass: Pass::Solid,
        ..ObjProperties::new_col(DARKGRAY)
    }
}
