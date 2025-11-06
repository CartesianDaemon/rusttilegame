use macroquad::prelude::*;

use crate::{engine::for_gamedata::*};
use super::movement_logic::ProgpuzzAI;

type CustomProps = super::ProgpuzzCustomProps;

pub fn new_progbot() -> FreeObj<CustomProps> {
    FreeObj {
        logical_props: LogicalProps::<CustomProps> {
            name:"Hero".to_string(),
            pass: Pass::Mov,
            custom_props: CustomProps {
                ai: ProgpuzzAI::Prog,
            },
            ..LogicalProps::<CustomProps>::defaults()
        },
        visual_props: VisualProps::new_text_fill("HERO".to_string(), Some(GOLD), Some(BLACK))
    }
}

pub fn new_floor() -> FreeObj<CustomProps> {
    FreeObj {
        logical_props: LogicalProps::<CustomProps> {
            name: "Floor".to_string(),
            ..LogicalProps::<CustomProps>::defaults()
        },
        visual_props: VisualProps::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> FreeObj<CustomProps> {
    FreeObj {
        logical_props: LogicalProps::<CustomProps> {
            name: "Wall".to_string(),
            pass: Pass::Solid,
            .. LogicalProps::<CustomProps>::defaults()
        },
        visual_props: VisualProps::new_col(DARKGRAY)
    }
}
