use std::collections::HashMap;

use super::objs::*;

use tile_engine::for_gamedata::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PushpuzzPaneId {
    NewGame,
    LevIntro(u16),
    LevArena(u16),
    LevOutro(u16),
    LevRetry(u16),
    Win,
}

#[derive(Debug)]
pub struct PushpuzzLevset {
    pub current_paneid: PushpuzzPaneId,
}

impl PushpuzzLevset {
    pub fn new() -> PushpuzzLevset {
        PushpuzzLevset { current_paneid: PushpuzzPaneId::NewGame }
    }

    pub fn advance_pane(&mut self, continuation: WidgetConclusion) {
        self.current_paneid = match (self.current_paneid, continuation) {
            (PushpuzzPaneId::NewGame, WidgetConclusion::SplashContinue) => PushpuzzPaneId::LevIntro(1),
            (PushpuzzPaneId::LevIntro(levnum), WidgetConclusion::SplashContinue) => PushpuzzPaneId::LevArena(levnum),
            (PushpuzzPaneId::LevArena(levnum), WidgetConclusion::Win) => PushpuzzPaneId::LevOutro(levnum),
            (PushpuzzPaneId::LevArena(levnum), WidgetConclusion::Die) => PushpuzzPaneId::LevRetry(levnum),
            (PushpuzzPaneId::LevRetry(levnum), WidgetConclusion::SplashContinue) => PushpuzzPaneId::LevArena(levnum),
            // TODO: Get max levnum from list of levels?
            (PushpuzzPaneId::LevOutro(2), WidgetConclusion::SplashContinue) => PushpuzzPaneId::Win,
            (PushpuzzPaneId::LevOutro(levnum), WidgetConclusion::SplashContinue) => PushpuzzPaneId::LevOutro(levnum+1),
            (PushpuzzPaneId::Win, WidgetConclusion::SplashContinue) => PushpuzzPaneId::NewGame,
            _ => panic!()
        };
    }

    pub fn load_pane(&self) -> Widget<super::PushpuzzGameLogic> {
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
            PushpuzzPaneId::NewGame => Widget::from_splash_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Hello!",
                    "Hi!",
                    "I'm a snake!",
                    "I'm crab!",
                ]
            ),

            PushpuzzPaneId::LevIntro(1) => {
                Widget::from_splash_string("Welcome to level 1!".to_string())
            },
            PushpuzzPaneId::LevArena(1) => Widget::from_play_ascii_map(&[
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
            PushpuzzPaneId::LevOutro(1) => {
                Widget::from_splash_string("Well done!! Goodbye from level 1".to_string())
            },

            PushpuzzPaneId::LevIntro(2) => {
                Widget::from_splash_string("Ooh, welcome to level 2!".to_string())
            },
            PushpuzzPaneId::LevArena(2) => Widget::from_play_ascii_map(&[
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
            PushpuzzPaneId::LevOutro(2) => {
                Widget::from_splash_string("Wow, well done!! Goodbye from level 2!".to_string())
            },

            PushpuzzPaneId::LevRetry(_levno) => {
                Widget::from_splash_string("Game Over. Press [enter] to retry.".to_string())
            },
            PushpuzzPaneId::Win => {
                Widget::from_splash_string("Congratulations. You win! Press [enter] to play again.".to_string())
            },

            PushpuzzPaneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            PushpuzzPaneId::LevArena(_) => panic!("Loading LevArena for level that can't be found."),
            PushpuzzPaneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
