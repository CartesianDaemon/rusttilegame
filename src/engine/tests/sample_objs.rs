// TODO: Move many of these tests into demo or into pushing puzzle??

use macroquad::prelude::*;

use crate::engine::for_scripting::*;

pub fn new_hero_crab() -> ObjProperties {
    ObjProperties {
        name:"Hero".to_string(),
        pass: Pass::Mov,
        ai: AI::Hero,
        ..ObjProperties::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_fish(dir: CoordDelta) -> ObjProperties {
    ObjProperties {
        name: "Fish".to_string(),
        pass: Pass::Mov,
        ai: AI::Bounce,
        dir: dir,
        effect: Effect::Kill,
        tex_scale: 1.7,
        ..ObjProperties::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
    }
}

pub fn new_gawpie(dir: CoordDelta) -> ObjProperties {
    ObjProperties {
        name: "Gawpie".to_string(),
        pass: Pass::Mov,
        ai: AI::Drift,
        dir: dir,
        effect: Effect::Kill,
        tex_scale: 1.7,
        ..ObjProperties::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
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

pub fn new_door_open() -> ObjProperties {
    ObjProperties {
        name: "OpenDoor".to_string(),
        ..ObjProperties::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> ObjProperties {
    ObjProperties {
        name: "ClosedDoor".to_string(),
        pass: Pass::Solid,
        ..ObjProperties::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> ObjProperties {
    ObjProperties {
        name: "Goal".to_string(),
        effect: Effect::Win,

        border: Some(LIGHTGRAY),
        ..ObjProperties::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
    }
}
