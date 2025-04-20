use std::collections::HashMap;

use super::objs::*;

use crate::engine::customgame::*;

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

    fn advance_scene(&mut self, continuation: SceneEnding) {
        self.current_sceneid = match (self.current_sceneid, continuation) {
            (BiobotSceneId::NewGame, SceneEnding::SplashNext) => BiobotSceneId::LevIntro(1),
            (BiobotSceneId::LevIntro(levnum), SceneEnding::SplashNext) => BiobotSceneId::LevPlay(levnum),
            (BiobotSceneId::LevPlay(levnum), SceneEnding::PlayWin) => BiobotSceneId::LevOutro(levnum),
            (BiobotSceneId::LevPlay(levnum), SceneEnding::PlayDie) => BiobotSceneId::LevRetry(levnum),
            (BiobotSceneId::LevRetry(levnum), SceneEnding::SplashNext) => BiobotSceneId::LevPlay(levnum),
            // TODO: Get max levnum from list of levels?
            (BiobotSceneId::LevOutro(2), SceneEnding::SplashNext) => BiobotSceneId::Win,
            (BiobotSceneId::LevOutro(levnum), SceneEnding::SplashNext) => BiobotSceneId::LevOutro(levnum+1),
            (BiobotSceneId::Win, SceneEnding::SplashNext) => BiobotSceneId::NewGame,
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
            BiobotSceneId::NewGame => Scene::from_splash_dialogue(
                //"Click or press [enter] to start.".to_string(),
                vec![
                    "Hello!",
                    "Hi!",
                    "I'm a snake!",
                    "I'm crab!",
                ]
            ),

            BiobotSceneId::LevIntro(1) => {
                Scene::from_splash_string("Welcome to level 1!".to_string())
            },
            BiobotSceneId::LevPlay(1) => Scene::from_play_ascii_map(&[
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
                Scene::from_splash_string("Well done!! Goodbye from level 1".to_string())
            },

            BiobotSceneId::LevIntro(2) => {
                Scene::from_splash_string("Ooh, welcome to level 2!".to_string())
            },
            BiobotSceneId::LevPlay(2) => Scene::from_play_ascii_map(&[
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
                Scene::from_splash_string("Wow, well done!! Goodbye from level 2!".to_string())
            },

            BiobotSceneId::LevRetry(_levno) => {
                Scene::from_splash_string("Game Over. Press [enter] to retry.".to_string())
            },
            BiobotSceneId::Win => {
                Scene::from_splash_string("Congratulations. You win! Press [enter] to play again.".to_string())
            },

            BiobotSceneId::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
            BiobotSceneId::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
            BiobotSceneId::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
        }
    }
}
