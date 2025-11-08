use std::collections::HashMap;

use super::objs::*;

use crate::for_gamedata::*;

// TOOD: Would it be useful to have a levset trait defining the necessary traits,
// even if it doesn't add any other functionality?
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
pub struct PushpuzzLevset {
    pub current_paneid: BiobotPaneId,
}

impl PushpuzzLevset {
    pub fn new() -> PushpuzzLevset {
        PushpuzzLevset { current_paneid: BiobotPaneId::NewGame }
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

    pub fn load_pane(&self) -> Pane<super::super::pushpuzz::PushpuzzMovementLogic> {
        let aquarium1_key = HashMap::from([
            // TODO: Combine with obj.char types?
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('>', vec![ new_floor(), new_fish(CoordDelta::from_xy(1,0)) ]),
            ('<', vec![ new_floor(), new_fish(CoordDelta::from_xy(-1,0)) ]),
            ('g', vec![ new_floor(), new_gawpie(CoordDelta::from_xy(1,0)) ]),
            ('h', vec![ new_floor(), new_hero_crab() ]),
            ('o', vec![ /* new_floor(), */ new_door_win() ]), // TODO: Check win on non-floor tiles
            ('@', vec![ new_floor(), new_door_closed() ]),
            ('_', vec![ new_door_open() ]), // Logically ought to be a floor, but for now draw movs over the 'door'
            /*
            */
        ]);

        match self.current_paneid {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            BiobotPaneId::NewGame => Pane::from_splash_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Hello!",
                    "Hi!",
                    "I'm a snake!",
                    "I'm crab!",
                ]
            ),

            BiobotPaneId::LevIntro(1) => {
                Pane::from_splash_string("Welcome to level 1!".to_string())
            },
            BiobotPaneId::LevArena(1) => Pane::from_play_ascii_map(&[
                "#            # #",
                "#####@####@###@#",
                "@              #",
                "#####_########_#",
                "#            # #",
                "#            # #",
                "#  >         @ @",
                "#            # #",
                "#            # #",
                "#       h    # #",
                "#            # o",
                "#            # #",
                "#            # #",
                "##############@#",
                "#            # #",
                "#            @ #",
            ], aquarium1_key),
            BiobotPaneId::LevOutro(1) => {
                Pane::from_splash_string("Well done!! Goodbye from level 1".to_string())
            },

            BiobotPaneId::LevIntro(2) => {
                Pane::from_splash_string("Ooh, welcome to level 2!".to_string())
            },
            BiobotPaneId::LevArena(2) => Pane::from_play_ascii_map(&[
                "################",
                "#              #",
                "#              #",
                "#              #",
                "#       h      #",
                "#              #",
                "#              #",
                "#  g           #",
                "#              #",
                "#        <     #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "#              #",
                "####o###########",
            ], aquarium1_key),
            BiobotPaneId::LevOutro(2) => {
                Pane::from_splash_string("Wow, well done!! Goodbye from level 2!".to_string())
            },

            BiobotPaneId::LevRetry(_levno) => {
                Pane::from_splash_string("Game Over. Press [enter] to retry.".to_string())
            },
            BiobotPaneId::Win => {
                Pane::from_splash_string("Congratulations. You win! Press [enter] to play again.".to_string())
            },

            BiobotPaneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotPaneId::LevArena(_) => panic!("Loading LevArena for level that can't be found."),
            BiobotPaneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
