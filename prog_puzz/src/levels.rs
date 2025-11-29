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
    pub current_levid: ProgpuzzPaneId,
}

impl ProgpuzzLevset {
    pub fn new() -> ProgpuzzLevset {
        ProgpuzzLevset { current_levid: ProgpuzzPaneId::LevCodingArena(1) }
    }

    pub fn advance_scene(&mut self, continuation: WidgetConclusion) {
        self.current_levid = match (self.current_levid, continuation) {
            // TODO: Get max levnum from list of levels?
            (ProgpuzzPaneId::LevCodingArena(1), WidgetConclusion::Win) => ProgpuzzPaneId::Win,
            (ProgpuzzPaneId::LevCodingArena(levnum), WidgetConclusion::Win) => ProgpuzzPaneId::LevCodingArena(levnum+1),
            (ProgpuzzPaneId::Win, WidgetConclusion::SplashContinue) => Self::new().current_levid,
            _ => panic!()
        };
    }

    pub fn load_scene(&self) -> Widget<super::game_logic::ProgpuzzGameLogic> {
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

        use ActionOp::*;
        use ParentOp::*;
        use Op::Action as A;
        use Op::Parent as P;

        let coding = if std::env::args().collect::<Vec<_>>().contains(&"--debug-coding=A".to_string()) {
            let mut coding = Coding::from_vec(&[
                (A(F), 1),
                (A(L), 1),
                (A(R), 1),
                (P(group), 1),
                (P(x2), 1),
            ]);

            coding.prog = Prog::from(vec![A(R), P(x2)]);
            coding.prog.instrs[1].subnodes = Some(Prog::from(vec![P(x2)]));
            coding.prog.instrs[1].subnodes.as_mut().unwrap().instrs[0].subnodes = Some(Prog::from(vec![A(F)]));

            coding
        } else if std::env::args().collect::<Vec<_>>().contains(&"--debug-coding=B".to_string()) {
            Coding::from_vec(&[
                (A(F), 2),
                (A(L), 2),
                (A(R), 2),
                (P(group), 2),
                (P(x2), 2),
                (P(loop5), 2),
            ])
        } else {
            let coding = Coding::from_vec(&[(A(F), 6), (A(R), 1)]);
            coding
        };

        // NB: Would like to implement thin walls between squares, not walls filling whole squares.
        match self.current_levid {
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
                coding,
            )),
            ProgpuzzPaneId::Win => {
                Widget::from_splash_string("Congratulations. You've completed all the levels. Press [enter] to play through again".to_string())
            },

            ProgpuzzPaneId::LevCodingArena(_) => panic!("Loading LevSplit for level that can't be found."),
        }
    }
}
