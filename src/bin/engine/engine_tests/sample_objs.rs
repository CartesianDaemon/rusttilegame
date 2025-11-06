// TODO: Move many of these tests into demo or into pushing puzzle??

use macroquad::prelude::*;

use crate::engine::for_gamedata::*;

type SimpleCustomProps = super::super::super::simple_custom_props::SimpleCustomProps;

pub fn new_hero_crab() -> FreeObj<SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name:"Hero".to_string(),
            pass: Pass::Mov,
            custom_props: SimpleCustomProps {
                ai: SimpleAI::Hero,
            },
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_fish(dir: CoordDelta) -> FreeObj<SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Fish".to_string(),
            pass: Pass::Mov,
            custom_props: SimpleCustomProps {
                ai: SimpleAI::Bounce,
            },
            dir: dir,
            effect: Effect::Kill,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps {
            tex_scale: 1.7,
            ..VisualProps::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
        }
    }
}

pub fn new_gawpie(dir: CoordDelta) -> FreeObj<SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Gawpie".to_string(),
            pass: Pass::Mov,
            custom_props: SimpleCustomProps {
                ai: SimpleAI::Drift,
            },
            dir: dir,
            effect: Effect::Kill,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps {
            tex_scale: 1.7,
            ..VisualProps::new_tex_anim(vec!["FishB.0001.png", "FishB.0002.png", "FishB.0003.png"])
        }
    }
}

pub fn new_floor() -> FreeObj<SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Floor".to_string(),
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> FreeObj<SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Wall".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col(DARKGRAY)
    }
}

pub fn new_door_open() -> FreeObj<SimpleCustomProps> {
    FreeObj {
            logical_props: LogicalProps::<SimpleCustomProps> {
            name: "OpenDoor".to_string(),
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> FreeObj<SimpleCustomProps> {
    FreeObj {
            logical_props: LogicalProps::<SimpleCustomProps> {
            name: "ClosedDoor".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> FreeObj<SimpleCustomProps> {
    FreeObj {
        logical_props: LogicalProps::<SimpleCustomProps> {
            name: "Goal".to_string(),
            effect: Effect::Win,
            .. LogicalProps::<SimpleCustomProps>::defaults()
        },
        visual_props: VisualProps{
            border: Some(LIGHTGRAY),
            ..VisualProps::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
        }
    }
}
