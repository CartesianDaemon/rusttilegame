use std::collections::HashMap;

use super::objs::*;

use crate::engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BiobotPaneId {
    NewGame,
    LevIntro(u16),
    LevArena(u16),
    LevOutro(u16),
    LevRetry(u16),
    Win,
}

#[derive(Debug)]
pub struct ProgpuzzLevset {
    pub current_paneid: BiobotPaneId,
}

impl ProgpuzzLevset {
    pub fn new() -> ProgpuzzLevset {
        ProgpuzzLevset { current_paneid: BiobotPaneId::NewGame }
    }

    pub fn advance_pane(&mut self, continuation: PaneConclusion) {
        self.current_paneid = match (self.current_paneid, continuation) {
            (BiobotPaneId::NewGame, PaneConclusion::SplashNext) => BiobotPaneId::LevIntro(1),
            (BiobotPaneId::LevIntro(levnum), PaneConclusion::SplashNext) => BiobotPaneId::LevArena(levnum),
            (BiobotPaneId::LevArena(levnum), PaneConclusion::ArenaWin) => BiobotPaneId::LevOutro(levnum),
            (BiobotPaneId::LevArena(levnum), PaneConclusion::ArenaDie) => BiobotPaneId::LevRetry(levnum),
            (BiobotPaneId::LevRetry(levnum), PaneConclusion::SplashNext) => BiobotPaneId::LevArena(levnum),
            // TODO: Get max levnum from list of levels?
            (BiobotPaneId::LevOutro(2), PaneConclusion::SplashNext) => BiobotPaneId::Win,
            (BiobotPaneId::LevOutro(levnum), PaneConclusion::SplashNext) => BiobotPaneId::LevOutro(levnum+1),
            (BiobotPaneId::Win, PaneConclusion::SplashNext) => BiobotPaneId::NewGame,
            _ => panic!()
        };
    }

    pub fn load_pane(&self) -> Pane<super::ProgpuzzMovementLogic> {
        let aquarium1_key = HashMap::from([
            // TODO: Combine with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('^', vec![ new_floor(), new_progbot() ]),
            /*
            */
        ]);

        match self.current_paneid {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            BiobotPaneId::NewGame => Pane::from_splash_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Welcome to programming bot game!",
                ]
            ),

            BiobotPaneId::LevIntro(1) => {
                Pane::from_splash_string("Welcome to level 1!".to_string())
            },
            BiobotPaneId::LevArena(1) => Pane::<super::ProgpuzzMovementLogic>::from_play_ascii_map(&[
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
            BiobotPaneId::LevOutro(1) => {
                Pane::from_splash_string("Well done!! Goodbye from level 1".to_string())
            },

            BiobotPaneId::LevRetry(_levno) => {
                Pane::from_splash_string("Press [enter] to restart.".to_string())
            },
            BiobotPaneId::Win => {
                Pane::from_splash_string("Congratulations. You've completed all the levels. Press [enter] to play through again".to_string())
            },

            BiobotPaneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotPaneId::LevArena(_) => panic!("Loading LevArena for level that can't be found."),
            BiobotPaneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
