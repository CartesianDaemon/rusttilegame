use std::collections::HashMap;

use crate::engine::*;
use crate::engine::{Scene, Continuation};

// TOOD: Would it be useful to have a levset trait defining the necessary traits,
// even if it doesn't add any other functionality?
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BiobotSceneId {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    LevRetry(u16),
    Win,
}

#[derive(Debug)]
pub struct BiobotGame {
    pub current_sceneid: BiobotSceneId,
}

impl GameTrait for BiobotGame {
    fn new_game() -> BiobotGame {
        BiobotGame { current_sceneid: BiobotSceneId::NewGame }
    }

    fn advance_scene(&mut self, continuation: Continuation) {
        self.current_sceneid = match (self.current_sceneid, continuation) {
            (BiobotSceneId::NewGame, Continuation::SplashContinue) => BiobotSceneId::LevIntro(1),
            (BiobotSceneId::LevIntro(levnum), Continuation::SplashContinue) => BiobotSceneId::LevPlay(levnum),
            (BiobotSceneId::LevPlay(levnum), Continuation::PlayWin) => BiobotSceneId::LevOutro(levnum),
            (BiobotSceneId::LevPlay(levnum), Continuation::PlayDie) => BiobotSceneId::LevRetry(levnum),
            (BiobotSceneId::LevRetry(levnum), Continuation::SplashContinue) => BiobotSceneId::LevPlay(levnum),
            // TODO: Get max levnum from list of levels?
            (BiobotSceneId::LevOutro(2), Continuation::SplashContinue) => BiobotSceneId::Win,
            (BiobotSceneId::LevOutro(levnum), Continuation::SplashContinue) => BiobotSceneId::LevOutro(levnum+1),
            (BiobotSceneId::Win, Continuation::SplashContinue) => BiobotSceneId::NewGame,
            _ => panic!()
        };
    }

    fn load_scene(&self) -> Scene {
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

        match self.current_sceneid {
            // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
            BiobotSceneId::NewGame => Scene::make_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Hello!",
                    "Hi!",
                    "I'm a snake!",
                    "I'm crab!",
                ]
            ),

            BiobotSceneId::LevIntro(1) => {
                Scene::make_splash("Welcome to level 1!".to_string())
            },
            BiobotSceneId::LevPlay(1) => Scene::play_from_ascii(&[
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
            BiobotSceneId::LevOutro(1) => {
                Scene::make_splash("Well done!! Goodbye from level 1".to_string())
            },

            BiobotSceneId::LevIntro(2) => {
                Scene::make_splash("Ooh, welcome to level 2!".to_string())
            },
            BiobotSceneId::LevPlay(2) => Scene::play_from_ascii(&[
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
            BiobotSceneId::LevOutro(2) => {
                Scene::make_splash("Wow, well done!! Goodbye from level 2!".to_string())
            },

            BiobotSceneId::LevRetry(_levno) => {
                Scene::make_splash("Game Over. Press [enter] to retry.".to_string())
            },
            BiobotSceneId::Win => {
                Scene::make_splash("Congratulations. You win! Press [enter] to play again.".to_string())
            },

            BiobotSceneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotSceneId::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
            BiobotSceneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
