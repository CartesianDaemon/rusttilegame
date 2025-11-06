use macroquad::prelude::*;

use crate::engine::for_gamedata::*;

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

pub fn new_fish(dir: CoordDelta) -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "Fish".to_string(),
            pass: Pass::Mov,
            ai: AI::Bounce,
            dir: dir,
            effect: Effect::Kill,
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps {
            tex_scale: 1.7,
            ..VisualProps::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
        }
    }
}

pub fn new_gawpie(dir: CoordDelta) -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "Gawpie".to_string(),
            pass: Pass::Mov,
            ai: AI::Drift,
            dir: dir,
            effect: Effect::Kill,
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps {
            tex_scale: 1.7,
            ..VisualProps::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
        }
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

pub fn new_door_open() -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
            logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "OpenDoor".to_string(),
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
            logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "ClosedDoor".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> FreeObj<super::simple_custom_props::SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<simple_custom_props::SimpleCustomProps> {
            name: "Goal".to_string(),
            effect: Effect::Win,
            .. LogicalProps::<simple_custom_props::SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps{
            border: Some(LIGHTGRAY),
            ..VisualProps::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
        }
    }
}
