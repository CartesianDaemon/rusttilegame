use std::collections::HashMap;

use super::objs::*;

use tile_engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProgpuzzPaneId {
    LevCodingArena(u16),
    Win,
}

#[derive(Debug)]
pub struct ProgpuzzLevset {
    pub current_paneid: ProgpuzzPaneId,
}

impl ProgpuzzLevset {
    pub fn new() -> ProgpuzzLevset {
        ProgpuzzLevset { current_paneid: ProgpuzzPaneId::LevCodingArena(1) }
    }

    pub fn advance_pane(&mut self, continuation: WidgetConclusion) {
        self.current_paneid = match (self.current_paneid, continuation) {
            // TODO: Get max levnum from list of levels?
            (ProgpuzzPaneId::LevCodingArena(1), WidgetConclusion::Win) => ProgpuzzPaneId::Win,
            (ProgpuzzPaneId::LevCodingArena(levnum), WidgetConclusion::Win) => ProgpuzzPaneId::LevCodingArena(levnum+1),
            (ProgpuzzPaneId::Win, WidgetConclusion::SplashContinue) => Self::new().current_paneid,
            _ => panic!()
        };
    }

    pub fn load_pane(&self) -> Widget<super::game_logic::ProgpuzzGameLogic> {
        let progpuzz_key = HashMap::from([
            // NB: Better to move this into obj? Combined with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('^', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, -1)) ]),
            ('>', vec![ new_floor(), new_progbot(CoordDelta::from_xy(1, 0)) ]),
            ('v', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, 1)) ]),
            ('<', vec![ new_floor(), new_progbot(CoordDelta::from_xy(-1, 0)) ]),
            ('w', vec![ new_door_win() ]),
            /*
            */
        ]);

        use Op::*;
        // NB: Would like to implement thin walls between squares, not walls filling whole squares.
        match self.current_paneid {
            // TODO: Avoid needing to specify HEIGHT explicitly.
            ProgpuzzPaneId::LevCodingArena(1) => Widget::CodingArena(CodingArena::new::<16>(
                Arena::from_map_and_key(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#       w      #",
                    "#              #",
                    "#              #",
                    "#     ^        #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], progpuzz_key),
                Coding::from_vec(
                    // NB: Consider crate macro to initialise vec
                    // NB: Consider my iteration macro here and elsewhere I collect'ed.
                    &[
                        (F, 6),
                        (R, 1),
                        // ("F", 1),
                        // ("L", 1),
                        // ("R", 1),
                        // ("Loop", 1),
                    ]
                )
            )),
            ProgpuzzPaneId::Win => {
                Widget::from_splash_string("Congratulations. You've completed all the levels. Press [enter] to play through again".to_string())
            },

            ProgpuzzPaneId::LevCodingArena(_) => panic!("Loading LevSplit for level that can't be found."),
        }
    }
}
