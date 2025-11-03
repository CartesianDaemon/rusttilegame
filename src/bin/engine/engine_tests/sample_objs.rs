// TODO: Move many of these tests into demo or into pushing puzzle??

use macroquad::prelude::*;

use crate::engine::for_scripting::*;

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

pub fn new_fish(dir: CoordDelta) -> FreeObj {
    FreeObj {
        logical_props: LogicalProps {
            name: "Fish".to_string(),
            pass: Pass::Mov,
            ai: AI::Bounce,
            dir: dir,
            effect: Effect::Kill,
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps {
            tex_scale: 1.7,
            ..VisualProps::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
        }
    }
}

pub fn new_gawpie(dir: CoordDelta) -> FreeObj {
    FreeObj {
        logical_props: LogicalProps {
            name: "Gawpie".to_string(),
            pass: Pass::Mov,
            ai: AI::Drift,
            dir: dir,
            effect: Effect::Kill,
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps {
            tex_scale: 1.7,
            ..VisualProps::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
        }
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

pub fn new_door_open() -> FreeObj {
    FreeObj {
            logical_props: LogicalProps {
            name: "OpenDoor".to_string(),
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> FreeObj {
    FreeObj {
            logical_props: LogicalProps {
            name: "ClosedDoor".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> FreeObj {
    FreeObj {
        logical_props: LogicalProps {
            name: "Goal".to_string(),
            effect: Effect::Win,
            .. LogicalProps::defaults()
        },
        visual_props: VisualProps{
            border: Some(LIGHTGRAY),
            ..VisualProps::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
        }
    }
}
