use std::collections::HashMap;

use super::objs::*;

use crate::engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BiobotPaneId {
    LevSplit(u16),
    Win,
}

#[derive(Debug)]
pub struct ProgpuzzLevset {
    pub current_paneid: BiobotPaneId,
}

impl ProgpuzzLevset {
    pub fn new() -> ProgpuzzLevset {
        ProgpuzzLevset { current_paneid: BiobotPaneId::LevSplit(1) }
    }

    pub fn advance_pane(&mut self, continuation: PaneConclusion) {
        self.current_paneid = match (self.current_paneid, continuation) {
            // TODO: Get max levnum from list of levels?
            (BiobotPaneId::LevSplit(1), PaneConclusion::ArenaWin) => BiobotPaneId::Win,
            (BiobotPaneId::LevSplit(levnum), PaneConclusion::ArenaWin) => BiobotPaneId::LevSplit(levnum+1),
            (BiobotPaneId::Win, PaneConclusion::SplashNext) => Self::new().current_paneid,
            _ => panic!()
        };
    }

    pub fn load_pane(&self) -> Pane<super::ProgpuzzMovementLogic> {
        let aquarium1_key = HashMap::from([
            // NB: Could it be combined with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('^', vec![ new_floor(), new_progbot() ]),
            /*
            */
        ]);

        // NB: Would like to implement thin walls between squares, not walls filling whole squares.
        match self.current_paneid {
            // TODO: Avoid needing to specify HEIGHT explicitly.
            BiobotPaneId::LevSplit(1) => Pane::Split(Split::new::<16>(
                Arena::from_ascii(&[
                    "################",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "#              #",
                    "################",
                ], aquarium1_key),
                Code::from_ascii(
                    // NB: Consider crate macro to initialise vec
                    // NB: Consider my iteration macro here and elsewhere I collect'ed.
                    [
                        ("F", 1),
                        ("L", 1),
                        ("R", 1),
                        ("Loop", 1),
                    ].into_iter().collect()
                )
            )),
            BiobotPaneId::Win => {
                Pane::from_splash_string("Congratulations. You've completed all the levels. Press [enter] to play through again".to_string())
            },

            BiobotPaneId::LevSplit(_) => panic!("Loading LevSplit for level that can't be found."),
        }
    }
}
