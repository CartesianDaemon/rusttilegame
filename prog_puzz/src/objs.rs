use macroquad::prelude::*;

use tile_engine::for_gamedata::*;
use super::game_logic::ProgpuzzAI;

type CustomProps = super::game_logic::ProgpuzzCustomProps;

pub fn new_progbot_with_prog(dir: CoordDelta, prog: &Prog) -> FreeObj<CustomProps> {
    FreeObj {
        logical_props: LogicalProps {
            name:"Progbot".to_string(),
            dir: dir,
            pass: Pass::Mov,
            custom_props: CustomProps {
                prog: prog.clone(),
                ..CustomProps::new(ProgpuzzAI::Prog)
            },
            ..LogicalProps::<CustomProps>::defaults()
        },
        visual_props: VisualProps::new_tex("ferris.png"),
    }
}

pub fn new_progbot(dir: CoordDelta) -> FreeObj<CustomProps> {
    new_progbot_with_prog(dir, &Prog::default())
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

pub fn new_door_win() -> FreeObj<CustomProps> {
    FreeObj {
        logical_props: LogicalProps::<CustomProps> {
            name: "Goal".to_string(),
            effect: Effect::Win,
            .. LogicalProps::<CustomProps>::defaults()
        },
        visual_props: VisualProps{
            border: Some(LIGHTGRAY),
            ..VisualProps::new_text_fill("EXIT".to_string(), Some(GOLD), Some(WHITE))
        }
    }
}
